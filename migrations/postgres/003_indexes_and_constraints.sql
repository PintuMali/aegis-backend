-- Performance indexes
CREATE INDEX idx_players_game_rating ON players(primary_game, aegis_rating DESC);
CREATE INDEX idx_players_team ON players(team_id) WHERE team_id IS NOT NULL;
CREATE INDEX idx_players_email ON players(email);
CREATE INDEX idx_players_username ON players(username);

CREATE INDEX idx_teams_game_status ON teams(primary_game, status);
CREATE INDEX idx_teams_captain ON teams(captain_id) WHERE captain_id IS NOT NULL;
CREATE INDEX idx_teams_rating ON teams(aegis_rating DESC);

CREATE INDEX idx_tournaments_game_status ON tournaments(game_type, status);
CREATE INDEX idx_tournaments_dates ON tournaments(start_date, end_date);
CREATE INDEX idx_tournaments_featured ON tournaments(featured) WHERE featured = true;

CREATE INDEX idx_tournament_teams_tournament ON tournament_teams(tournament_id);
CREATE INDEX idx_tournament_teams_team ON tournament_teams(team_id);

CREATE INDEX idx_matches_tournament ON matches(tournament_id, match_number);
CREATE INDEX idx_matches_status_date ON matches(status, scheduled_start);

CREATE INDEX idx_player_stats_game ON player_game_stats(game_type, player_id);
CREATE INDEX idx_transactions_player ON transactions(player_id, created_at DESC);

-- Add foreign key constraints
ALTER TABLE players ADD CONSTRAINT fk_players_team 
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE SET NULL;

ALTER TABLE teams ADD CONSTRAINT fk_teams_captain 
    FOREIGN KEY (captain_id) REFERENCES players(id) ON DELETE SET NULL;

-- Add check constraints for data integrity
ALTER TABLE players ADD CONSTRAINT chk_players_age 
    CHECK (age IS NULL OR (age >= 13 AND age <= 99));

ALTER TABLE players ADD CONSTRAINT chk_players_coins 
    CHECK (coins >= 0);

ALTER TABLE teams ADD CONSTRAINT chk_teams_rating 
    CHECK (aegis_rating >= 0 AND aegis_rating <= 5000);

ALTER TABLE tournaments ADD CONSTRAINT chk_tournaments_dates 
    CHECK (end_date > start_date);

ALTER TABLE tournaments ADD CONSTRAINT chk_tournaments_max_teams 
    CHECK (max_teams > 0 AND max_teams <= 100);
