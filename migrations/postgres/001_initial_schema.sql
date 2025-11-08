-- Core game types and enums
CREATE TYPE game_type AS ENUM ('BGMI', 'VALORANT', 'CS2', 'APEX', 'FORTNITE', 'LOL', 'DOTA2');
CREATE TYPE tournament_status AS ENUM ('announced', 'registration_open', 'registration_closed', 'in_progress', 'completed', 'cancelled');
CREATE TYPE team_status AS ENUM ('active', 'inactive', 'disbanded', 'looking_for_players');
CREATE TYPE match_status AS ENUM ('scheduled', 'in_progress', 'completed', 'cancelled');

-- Players table
CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    real_name VARCHAR(100),
    in_game_name VARCHAR(100),
    primary_game game_type NOT NULL,
    aegis_rating INTEGER DEFAULT 1000,
    country VARCHAR(3),
    age SMALLINT CHECK (age >= 13 AND age <= 99),
    coins BIGINT DEFAULT 0 CHECK (coins >= 0),
    verified BOOLEAN DEFAULT FALSE,
    profile_picture TEXT,
    bio TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Teams table
CREATE TABLE teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) UNIQUE NOT NULL,
    tag VARCHAR(10) UNIQUE NOT NULL,
    logo TEXT,
    captain_id UUID REFERENCES players(id),
    primary_game game_type NOT NULL,
    region VARCHAR(50),
    status team_status DEFAULT 'active',
    total_earnings DECIMAL(15,2) DEFAULT 0,
    aegis_rating INTEGER DEFAULT 1000,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Organizations table
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) UNIQUE NOT NULL,
    owner_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    country VARCHAR(3) NOT NULL,
    logo TEXT,
    description TEXT,
    verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tournaments table
CREATE TABLE tournaments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    short_name VARCHAR(50),
    game_type game_type NOT NULL,
    status tournament_status DEFAULT 'announced',
    organizer_id UUID REFERENCES organizations(id),
    prize_pool_amount DECIMAL(15,2) DEFAULT 0,
    prize_pool_currency VARCHAR(3) DEFAULT 'INR',
    max_teams INTEGER NOT NULL CHECK (max_teams > 0),
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    registration_start TIMESTAMPTZ,
    registration_end TIMESTAMPTZ,
    rules JSONB,
    settings JSONB,
    featured BOOLEAN DEFAULT FALSE,
    verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
