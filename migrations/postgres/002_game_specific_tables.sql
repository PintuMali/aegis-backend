-- Game-specific player statistics (scalable for multiple games)
CREATE TABLE player_game_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    game_type game_type NOT NULL,
    rank_tier VARCHAR(50),
    matches_played INTEGER DEFAULT 0,
    wins INTEGER DEFAULT 0,
    kills INTEGER DEFAULT 0,
    deaths INTEGER DEFAULT 0,
    assists INTEGER DEFAULT 0,
    damage_dealt BIGINT DEFAULT 0,
    stats JSONB, -- Game-specific stats (K/D, ADR, etc.)
    season VARCHAR(20),
    last_updated TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(player_id, game_type, season)
);

-- Tournament teams junction table
CREATE TABLE tournament_teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID REFERENCES tournaments(id) ON DELETE CASCADE,
    team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    qualified_through VARCHAR(50),
    current_stage VARCHAR(100),
    total_points INTEGER DEFAULT 0,
    total_kills INTEGER DEFAULT 0,
    placement INTEGER,
    prize_amount DECIMAL(15,2),
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(tournament_id, team_id)
);

-- Matches table
CREATE TABLE matches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID REFERENCES tournaments(id) ON DELETE CASCADE,
    match_number INTEGER NOT NULL,
    game_type game_type NOT NULL,
    status match_status DEFAULT 'scheduled',
    map_name VARCHAR(50),
    scheduled_start TIMESTAMPTZ NOT NULL,
    actual_start TIMESTAMPTZ,
    actual_end TIMESTAMPTZ,
    settings JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Match participants (teams in a match)
CREATE TABLE match_participants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    match_id UUID REFERENCES matches(id) ON DELETE CASCADE,
    team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    placement INTEGER,
    kills INTEGER DEFAULT 0,
    points INTEGER DEFAULT 0,
    stats JSONB, -- Match-specific team stats
    UNIQUE(match_id, team_id)
);

-- Financial transactions (for prize money, coins)
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    tournament_id UUID REFERENCES tournaments(id),
    transaction_type VARCHAR(50) NOT NULL, -- 'prize', 'coins', 'entry_fee'
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'INR',
    status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'completed', 'failed'
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    processed_at TIMESTAMPTZ
);
