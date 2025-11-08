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
    stats JSONB,
    UNIQUE(match_id, team_id)
);

CREATE INDEX idx_matches_tournament ON matches(tournament_id, match_number);
CREATE INDEX idx_matches_status_date ON matches(status, scheduled_start);
CREATE INDEX idx_match_participants_match ON match_participants(match_id);
