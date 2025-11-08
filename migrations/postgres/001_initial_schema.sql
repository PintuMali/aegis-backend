-- Multi-game scalable enums
CREATE TYPE game_type AS ENUM ('BGMI', 'VALORANT', 'CS2', 'APEX', 'FORTNITE', 'LOL', 'DOTA2', 'PUBG', 'COD');
CREATE TYPE tournament_status AS ENUM ('announced', 'registration_open', 'registration_closed', 'in_progress', 'completed', 'cancelled', 'postponed');
CREATE TYPE team_status AS ENUM ('active', 'inactive', 'disbanded', 'looking_for_players');
CREATE TYPE battle_status AS ENUM ('scheduled', 'in_progress', 'completed', 'cancelled');
CREATE TYPE approval_status AS ENUM ('pending', 'approved', 'rejected', 'not_applicable');
CREATE TYPE admin_role AS ENUM ('super_admin', 'admin', 'moderator');

-- Core Players 
CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cognito_sub VARCHAR(255) UNIQUE DEFAULT NULL, 
    username VARCHAR(50) UNIQUE NOT NULL,
    in_game_name VARCHAR(100),
    real_name VARCHAR(100),
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL, -- Current auth method
    reset_password_token VARCHAR(255),
    reset_password_expiry TIMESTAMPTZ,
    verified BOOLEAN DEFAULT FALSE,
    country VARCHAR(100),
    bio TEXT DEFAULT '',
    profile_picture TEXT DEFAULT '',
    primary_game game_type,
    earnings DECIMAL(15,2) DEFAULT 0,
    in_game_role TEXT[],
    location VARCHAR(100),
    age INTEGER CHECK (age >= 13 AND age <= 99),
    languages TEXT[],
    aegis_rating INTEGER DEFAULT 0,
    tournaments_played INTEGER DEFAULT 0,
    battles_played INTEGER DEFAULT 0, 
    qualified_events BOOLEAN DEFAULT FALSE,
    qualified_event_details TEXT[],
    team_status VARCHAR(50),
    team_id UUID,
    availability VARCHAR(50),
    discord_tag VARCHAR(100) DEFAULT '',
    twitch VARCHAR(255) DEFAULT '',
    youtube VARCHAR(255) DEFAULT '',
    twitter VARCHAR(255) DEFAULT '',
    profile_visibility VARCHAR(20) DEFAULT 'public',
    card_theme VARCHAR(20) DEFAULT 'orange',
    coins BIGINT DEFAULT 0,
    last_check_in TIMESTAMPTZ,
    check_in_streak INTEGER DEFAULT 0,
    total_check_ins INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Game-specific player stats (scalable for all games)
CREATE TABLE player_game_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    game_type game_type NOT NULL,
    rank_tier VARCHAR(50),
    battles_played INTEGER DEFAULT 0, 
    wins INTEGER DEFAULT 0,
    kills INTEGER DEFAULT 0,
    game_specific_stats JSONB DEFAULT '{}',
    last_updated TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(player_id, game_type)
);

-- Teams 
CREATE TABLE teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_name VARCHAR(100) UNIQUE NOT NULL,
    team_tag VARCHAR(5) UNIQUE,
    logo TEXT DEFAULT 'https://placehold.co/200x200/1a1a1a/ffffff?text=TEAM',
    captain UUID, 
    primary_game game_type DEFAULT 'BGMI',
    region VARCHAR(50) DEFAULT 'India',
    country VARCHAR(100),
    bio TEXT DEFAULT '',
    established_date TIMESTAMPTZ DEFAULT NOW(),
    total_earnings DECIMAL(15,2) DEFAULT 0,
    aegis_rating INTEGER DEFAULT 0,
    organization_id UUID,
    discord VARCHAR(255) DEFAULT '',
    twitter VARCHAR(255) DEFAULT '',
    twitch VARCHAR(255) DEFAULT '',
    youtube VARCHAR(255) DEFAULT '',
    website VARCHAR(255) DEFAULT '',
    profile_visibility VARCHAR(20) DEFAULT 'public',
    status team_status DEFAULT 'active',
    looking_for_players BOOLEAN DEFAULT FALSE,
    open_roles TEXT[],
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Organizations 
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cognito_sub VARCHAR(255) UNIQUE DEFAULT NULL, 
    org_name VARCHAR(200) UNIQUE NOT NULL,
    owner_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    google_id VARCHAR(255),
    password VARCHAR(255) NOT NULL, -- Current auth method
    country VARCHAR(100) NOT NULL,
    headquarters VARCHAR(200),
    description TEXT DEFAULT '',
    logo TEXT DEFAULT '',
    established_date TIMESTAMPTZ DEFAULT NOW(),
    active_games game_type[] DEFAULT '{}',
    total_earnings DECIMAL(15,2) DEFAULT 0,
    contact_phone VARCHAR(20) DEFAULT '',
    discord VARCHAR(255) DEFAULT '',
    twitter VARCHAR(255) DEFAULT '',
    twitch VARCHAR(255) DEFAULT '',
    youtube VARCHAR(255) DEFAULT '',
    website VARCHAR(255) DEFAULT '',
    linkedin VARCHAR(255) DEFAULT '',
    profile_visibility VARCHAR(20) DEFAULT 'public',
    approval_status approval_status DEFAULT 'pending',
    approved_by UUID,
    approval_date TIMESTAMPTZ,
    rejection_reason TEXT,
    email_verified BOOLEAN DEFAULT FALSE,
    verification_token VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Admins 
CREATE TABLE admins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    role admin_role DEFAULT 'admin',
    permissions JSONB DEFAULT '{}',
    is_active BOOLEAN DEFAULT TRUE,
    last_login TIMESTAMPTZ,
    login_attempts INTEGER DEFAULT 0,
    lock_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tournaments 
CREATE TABLE tournaments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_name VARCHAR(150) UNIQUE NOT NULL,
    short_name VARCHAR(50),
    slug VARCHAR(200) UNIQUE,
    game_title VARCHAR(50) DEFAULT 'BGMI',
    tier VARCHAR(20) DEFAULT 'Community',
    region VARCHAR(50) DEFAULT 'India',
    sub_region VARCHAR(100),
    organizer JSONB DEFAULT '{}',
    sponsors JSONB DEFAULT '[]',
    announcement_date TIMESTAMPTZ,
    is_open_for_all BOOLEAN DEFAULT FALSE,
    registration_start_date TIMESTAMPTZ,
    registration_end_date TIMESTAMPTZ,
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    status tournament_status DEFAULT 'announced',
    format VARCHAR(100),
    format_details TEXT,
    slots JSONB DEFAULT '{}',
    participating_teams JSONB DEFAULT '[]',
    phases JSONB DEFAULT '[]',
    final_standings JSONB DEFAULT '[]',
    prize_pool JSONB DEFAULT '{}',
    statistics JSONB DEFAULT '{}',
    awards JSONB DEFAULT '[]',
    media JSONB DEFAULT '{}',
    stream_links JSONB DEFAULT '[]',
    social_media JSONB DEFAULT '{}',
    description TEXT,
    ruleset_document TEXT,
    website_link TEXT,
    game_settings JSONB DEFAULT '{}',
    visibility VARCHAR(20) DEFAULT 'public',
    featured BOOLEAN DEFAULT FALSE,
    verified BOOLEAN DEFAULT FALSE,
    parent_series UUID,
    qualifies_for JSONB DEFAULT '[]',
    tags TEXT[],
    notes TEXT,
    external_ids JSONB DEFAULT '{}',
    approval_status approval_status DEFAULT 'not_applicable',
    submitted_by UUID,
    submitted_at TIMESTAMPTZ,
    approved_by UUID,
    approved_at TIMESTAMPTZ,
    rejected_by UUID,
    rejected_at TIMESTAMPTZ,
    rejection_reason TEXT,
    pending_invitations JSONB DEFAULT '[]',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tournament Teams 
CREATE TABLE tournament_teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID REFERENCES tournaments(id) ON DELETE CASCADE,
    team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    qualified_through VARCHAR(50),
    current_stage VARCHAR(100),
    total_tournament_points INTEGER DEFAULT 0,
    total_tournament_kills INTEGER DEFAULT 0,
    final_placement INTEGER,
    prize_amount DECIMAL(15,2),
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(tournament_id, team_id)
);

-- Battles 
CREATE TABLE battles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    battle_number INTEGER NOT NULL,
    tournament UUID REFERENCES tournaments(id) ON DELETE CASCADE,
    tournament_phase VARCHAR(200),
    scheduled_start_time TIMESTAMPTZ NOT NULL,
    status battle_status DEFAULT 'scheduled',
    map VARCHAR(50),
    participating_groups TEXT[],
    participating_teams JSONB DEFAULT '[]',
    battle_stats JSONB DEFAULT '{}',
    stream_urls JSONB DEFAULT '[]',
    room_credentials JSONB DEFAULT '{}',
    points_system JSONB DEFAULT '{}',
    tags TEXT[],
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tournament Team Invites 
CREATE TABLE tournament_team_invites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament UUID REFERENCES tournaments(id) ON DELETE CASCADE,
    team UUID REFERENCES teams(id) ON DELETE CASCADE,
    phase VARCHAR(200) NOT NULL,
    organizer UUID REFERENCES organizations(id) ON DELETE CASCADE,
    status VARCHAR(20) DEFAULT 'pending',
    message TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Financial Transactions (ACID compliance)
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    tournament_id UUID REFERENCES tournaments(id),
    transaction_type VARCHAR(50) NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'INR',
    status VARCHAR(20) DEFAULT 'pending',
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    processed_at TIMESTAMPTZ
);

-- Rewards 
CREATE TABLE rewards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    points INTEGER NOT NULL CHECK (points >= 0),
    description TEXT DEFAULT '',
    image TEXT DEFAULT '',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Performance Indexes
CREATE INDEX idx_players_game_rating ON players(primary_game, aegis_rating DESC);
CREATE INDEX idx_players_team ON players(team_id) WHERE team_id IS NOT NULL;
CREATE INDEX idx_players_email ON players(email);
CREATE INDEX idx_players_username ON players(username);

CREATE INDEX idx_teams_game_status ON teams(primary_game, status);
CREATE INDEX idx_teams_captain ON teams(captain) WHERE captain IS NOT NULL;
CREATE INDEX idx_teams_rating ON teams(aegis_rating DESC);

CREATE INDEX idx_tournaments_game_status ON tournaments(game_title, status);
CREATE INDEX idx_tournaments_dates ON tournaments(start_date, end_date);
CREATE INDEX idx_tournaments_featured ON tournaments(featured) WHERE featured = TRUE;

CREATE INDEX idx_battles_tournament ON battles(tournament, battle_number);
CREATE INDEX idx_battles_status ON battles(status, scheduled_start_time);

CREATE INDEX idx_player_stats_game ON player_game_stats(game_type, player_id);
CREATE INDEX idx_transactions_player ON transactions(player_id, created_at DESC);

-- Foreign Key Constraints
ALTER TABLE players ADD CONSTRAINT fk_players_team 
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE SET NULL;

ALTER TABLE teams ADD CONSTRAINT fk_teams_captain 
    FOREIGN KEY (captain) REFERENCES players(id) ON DELETE SET NULL;

ALTER TABLE teams ADD CONSTRAINT fk_teams_organization 
    FOREIGN KEY (organization_id) REFERENCES organizations(id) ON DELETE SET NULL;

-- Data Integrity Constraints
ALTER TABLE players ADD CONSTRAINT chk_players_age 
    CHECK (age IS NULL OR (age >= 13 AND age <= 99));

ALTER TABLE players ADD CONSTRAINT chk_players_coins 
    CHECK (coins >= 0);

ALTER TABLE teams ADD CONSTRAINT chk_teams_rating 
    CHECK (aegis_rating >= 0 AND aegis_rating <= 5000);

ALTER TABLE tournaments ADD CONSTRAINT chk_tournaments_dates 
    CHECK (end_date > start_date);

ALTER TABLE rewards ADD CONSTRAINT chk_rewards_points 
    CHECK (points >= 0);
