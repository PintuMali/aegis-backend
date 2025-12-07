-- Multi-game scalable enums
CREATE TYPE game_type AS ENUM ('BGMI', 'VALORANT', 'CS2', 'APEX', 'FORTNITE', 'LOL', 'DOTA2', 'PUBG', 'COD');
CREATE TYPE tournament_status AS ENUM ('announced', 'registration_open', 'registration_closed', 'in_progress', 'completed', 'cancelled', 'postponed');
CREATE TYPE team_status AS ENUM ('active', 'inactive', 'disbanded', 'looking_for_players');
CREATE TYPE battle_status AS ENUM ('scheduled', 'in_progress', 'completed', 'cancelled');
CREATE TYPE approval_status AS ENUM ('pending', 'approved', 'rejected', 'not_applicable');
CREATE TYPE admin_role AS ENUM ('super_admin', 'admin', 'moderator');

-- Core Players (UNCHANGED - All existing columns preserved)
CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    in_game_name VARCHAR(100),
    real_name VARCHAR(100),
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
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

-- Game-specific player stats (UNCHANGED)
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

-- Teams (UNCHANGED)
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

-- Organizations (UNCHANGED)
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_name VARCHAR(200) UNIQUE NOT NULL,
    owner_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
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
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Admins (UNCHANGED)
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

-- 🚀 ENTERPRISE AUTH: User Sessions (Stripe/GitHub/AWS Pattern)
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    session_token VARCHAR(255) UNIQUE NOT NULL,
    refresh_token VARCHAR(255) UNIQUE NOT NULL,
    user_type VARCHAR(20) NOT NULL CHECK (user_type IN ('player', 'admin', 'organization')),
    ip_address VARCHAR(45),
    user_agent TEXT,
    device_fingerprint VARCHAR(255),
    expires_at TIMESTAMPTZ NOT NULL,
    revoked BOOLEAN DEFAULT FALSE,
    revoked_at TIMESTAMPTZ,
    revoked_reason VARCHAR(100),
    last_activity TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 🛡️ ENTERPRISE SECURITY: Audit Trail (SOC2/GDPR Compliance)
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID,
    user_type VARCHAR(20),
    session_id UUID,
    action VARCHAR(50) NOT NULL,
    resource VARCHAR(100),
    resource_id UUID,
    ip_address VARCHAR(45),
    user_agent TEXT,
    success BOOLEAN NOT NULL,
    failure_reason VARCHAR(255),
    request_id VARCHAR(255),
    details JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 🔒 ENTERPRISE SECURITY: Rate Limiting (DDoS Protection)
CREATE TABLE rate_limits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    identifier VARCHAR(255) NOT NULL,
    identifier_type VARCHAR(20) NOT NULL CHECK (identifier_type IN ('ip', 'user_id', 'email')),
    action VARCHAR(50) NOT NULL,
    attempts INTEGER DEFAULT 1,
    window_start TIMESTAMPTZ DEFAULT NOW(),
    blocked_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(identifier, identifier_type, action)
);

-- 🔐 ENTERPRISE SECURITY: API Keys (Service-to-Service Auth)
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key_id VARCHAR(50) UNIQUE NOT NULL,
    key_hash VARCHAR(255) NOT NULL,
    name VARCHAR(100) NOT NULL,
    owner_id UUID NOT NULL,
    owner_type VARCHAR(20) NOT NULL CHECK (owner_type IN ('admin', 'organization')),
    scopes TEXT[] DEFAULT '{}',
    rate_limit_per_hour INTEGER DEFAULT 1000,
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tournaments (UNCHANGED - All existing columns preserved)
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

-- Tournament Teams (UNCHANGED)
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

-- Battles (UNCHANGED)
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

-- Tournament Team Invites (UNCHANGED)
CREATE TABLE tournament_team_invites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament UUID REFERENCES tournaments(id) ON DELETE CASCADE,
    team UUID REFERENCES teams(id) ON DELETE CASCADE,
    phase VARCHAR(200) NOT NULL,
    organizer UUID REFERENCES organizations(id) ON DELETE CASCADE,
    status VARCHAR(20) DEFAULT 'pending',
    message TEXT,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Rewards (UNCHANGED)
CREATE TABLE rewards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reward_name VARCHAR(100) NOT NULL,
    reward_type VARCHAR(50) NOT NULL,
    description TEXT,
    value DECIMAL(10,2),
    currency VARCHAR(10),
    requirements JSONB DEFAULT '{}',
    availability_start TIMESTAMPTZ,
    availability_end TIMESTAMPTZ,
    max_claims INTEGER,
    current_claims INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Transactions (UNCHANGED)
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    transaction_type VARCHAR(50) NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(10) DEFAULT 'USD',
    description TEXT,
    reference_id VARCHAR(255),
    status VARCHAR(20) DEFAULT 'pending',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ==========================================
-- CHAT SYSTEM TABLES (Enterprise Grade)
-- ==========================================

-- Chat Rooms/Channels
CREATE TABLE chats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chat_type VARCHAR(20) NOT NULL CHECK (chat_type IN ('general', 'team', 'tournament', 'community', 'direct')),
    name VARCHAR(100) NOT NULL,
    description TEXT DEFAULT '',
    participants JSONB DEFAULT '[]', -- Array of player UUIDs
    created_by UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    tournament_id UUID REFERENCES tournaments(id) ON DELETE CASCADE,
    community_id UUID,
    is_private BOOLEAN DEFAULT FALSE,
    max_participants INTEGER DEFAULT 1000,
    settings JSONB DEFAULT '{}', -- Chat settings, permissions
    metadata JSONB DEFAULT '{}', -- Additional data
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Chat Messages (High Performance with Direct Message Support)
CREATE TABLE chat_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chat_id UUID REFERENCES chats(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    receiver_id UUID REFERENCES players(id) ON DELETE CASCADE, -- Fixed syntax error
    message TEXT NOT NULL,
    message_type VARCHAR(20) DEFAULT 'text' CHECK (message_type IN ('text', 'image', 'file', 'system', 'emoji')),
    reply_to UUID REFERENCES chat_messages(id) ON DELETE SET NULL,
    attachments JSONB DEFAULT '[]', -- File URLs, metadata
    reactions JSONB DEFAULT '{}', -- {emoji: [user_ids]}
    edited_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Enterprise constraint: Either group message OR direct message
    CONSTRAINT check_chat_or_receiver 
        CHECK ((chat_id IS NOT NULL AND receiver_id IS NULL) OR (chat_id IS NULL AND receiver_id IS NOT NULL))
);


-- ==========================================
-- COMMUNITIES SYSTEM TABLES
-- ==========================================

-- Communities (Gaming Groups/Clans)
CREATE TABLE communities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) UNIQUE NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    description TEXT DEFAULT '',
    avatar TEXT DEFAULT '',
    banner TEXT DEFAULT '',
    game_focus game_type,
    region VARCHAR(50) DEFAULT 'Global',
    privacy VARCHAR(20) DEFAULT 'public' CHECK (privacy IN ('public', 'private', 'invite_only')),
    member_count INTEGER DEFAULT 0,
    max_members INTEGER DEFAULT 10000,
    owner_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    moderators JSONB DEFAULT '[]', -- Array of player UUIDs
    rules JSONB DEFAULT '[]', -- Community rules
    tags TEXT[] DEFAULT '{}',
    settings JSONB DEFAULT '{}', -- Community settings
    stats JSONB DEFAULT '{}', -- Activity stats
    social_links JSONB DEFAULT '{}', -- Discord, Twitter, etc.
    verified BOOLEAN DEFAULT FALSE,
    featured BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Community Posts (Social Feed)
CREATE TABLE community_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    title VARCHAR(200),
    content TEXT NOT NULL,
    post_type VARCHAR(20) DEFAULT 'text' CHECK (post_type IN ('text', 'image', 'video', 'poll', 'event', 'announcement')),
    attachments JSONB DEFAULT '[]', -- Media files
    poll_data JSONB DEFAULT '{}', -- Poll options, votes
    event_data JSONB DEFAULT '{}', -- Event details
    tags TEXT[] DEFAULT '{}',
    upvotes INTEGER DEFAULT 0,
    downvotes INTEGER DEFAULT 0,
    comment_count INTEGER DEFAULT 0,
    view_count INTEGER DEFAULT 0,
    pinned BOOLEAN DEFAULT FALSE,
    locked BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMPTZ,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Post Comments (Nested Comments Support)
CREATE TABLE post_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES community_posts(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES post_comments(id) ON DELETE CASCADE, -- For nested comments
    content TEXT NOT NULL,
    upvotes INTEGER DEFAULT 0,
    downvotes INTEGER DEFAULT 0,
    reply_count INTEGER DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Community Memberships
CREATE TABLE community_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    role VARCHAR(20) DEFAULT 'member' CHECK (role IN ('owner', 'moderator', 'member', 'banned')),
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    banned_until TIMESTAMPTZ,
    ban_reason TEXT,
    permissions JSONB DEFAULT '{}',
    UNIQUE(community_id, player_id)
);

-- Activity Logs (Enterprise Audit Trail)
CREATE TABLE activity_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_type VARCHAR(20) NOT NULL CHECK (entity_type IN ('chat', 'community', 'post', 'comment')),
    entity_id UUID NOT NULL,
    actor_id UUID REFERENCES players(id) ON DELETE SET NULL,
    action VARCHAR(50) NOT NULL,
    details JSONB DEFAULT '{}',
    ip_address VARCHAR(45),
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Team player invitations (enterprise standard)
CREATE TABLE team_player_invitations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    invited_player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    inviter_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'accepted', 'declined', 'expired')),
    message TEXT,
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '7 days'),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(team_id, invited_player_id, status) -- Prevent duplicate pending invites
);

-- Enterprise indexes for performance
CREATE INDEX idx_team_player_invitations_invited_player ON team_player_invitations(invited_player_id, status);
CREATE INDEX idx_team_player_invitations_team ON team_player_invitations(team_id, status);
CREATE INDEX idx_team_player_invitations_expires ON team_player_invitations(expires_at) WHERE status = 'pending';

-- Auto-update timestamp trigger
CREATE TRIGGER update_team_player_invitations_updated_at
    BEFORE UPDATE ON team_player_invitations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();


-- 🚀 ENTERPRISE PERFORMANCE: Optimized Indexes
CREATE INDEX idx_players_email ON players(email);
CREATE INDEX idx_players_username ON players(username);
CREATE INDEX idx_players_verified ON players(verified);
CREATE INDEX idx_players_team_id ON players(team_id);

CREATE INDEX idx_organizations_email ON organizations(email);
CREATE INDEX idx_organizations_approval_status ON organizations(approval_status);

CREATE INDEX idx_admins_email ON admins(email);
CREATE INDEX idx_admins_is_active ON admins(is_active);

-- Enterprise Auth Indexes (Fixed IMMUTABLE issues)
CREATE INDEX idx_user_sessions_active ON user_sessions(user_id, user_type, expires_at) WHERE NOT revoked;
CREATE INDEX idx_user_sessions_token ON user_sessions(session_token) WHERE NOT revoked;
CREATE INDEX idx_user_sessions_refresh ON user_sessions(refresh_token) WHERE NOT revoked;
CREATE INDEX idx_user_sessions_cleanup ON user_sessions(expires_at) WHERE NOT revoked;
CREATE INDEX idx_user_sessions_activity ON user_sessions(last_activity DESC);

CREATE INDEX idx_audit_logs_user ON audit_logs(user_id, user_type, created_at DESC);
CREATE INDEX idx_audit_logs_action ON audit_logs(action, created_at DESC);
CREATE INDEX idx_audit_logs_session ON audit_logs(session_id);

-- Fixed IMMUTABLE issues
CREATE INDEX idx_rate_limits_active ON rate_limits(identifier, identifier_type, action, blocked_until);
CREATE INDEX idx_rate_limits_cleanup ON rate_limits(window_start, blocked_until);

CREATE INDEX idx_api_keys_lookup ON api_keys(key_id) WHERE is_active;
CREATE INDEX idx_api_keys_owner ON api_keys(owner_id, owner_type) WHERE is_active;

CREATE INDEX idx_tournaments_status ON tournaments(status);
CREATE INDEX idx_tournaments_game_title ON tournaments(game_title);
CREATE INDEX idx_tournaments_start_date ON tournaments(start_date);

CREATE INDEX idx_tournament_teams_tournament_id ON tournament_teams(tournament_id);
CREATE INDEX idx_tournament_teams_team_id ON tournament_teams(team_id);

CREATE INDEX idx_battles_tournament ON battles(tournament);
CREATE INDEX idx_battles_status ON battles(status);

CREATE INDEX idx_transactions_player_id ON transactions(player_id);
CREATE INDEX idx_transactions_status ON transactions(status);

-- Chat System Indexes (Critical for Real-time)
CREATE INDEX idx_chats_type_participants ON chats USING GIN (participants);
CREATE INDEX idx_chats_type ON chats(chat_type);
CREATE INDEX idx_chats_created_by ON chats(created_by);
CREATE INDEX idx_chats_team_tournament ON chats(team_id, tournament_id) WHERE team_id IS NOT NULL OR tournament_id IS NOT NULL;

-- Chat Messages (Optimized for pagination)
CREATE INDEX idx_chat_messages_chat_time ON chat_messages(chat_id, created_at DESC);
CREATE INDEX idx_chat_messages_sender ON chat_messages(sender_id, created_at DESC);
CREATE INDEX idx_chat_messages_receiver ON chat_messages(receiver_id, created_at DESC) WHERE receiver_id IS NOT NULL;
CREATE INDEX idx_chat_messages_active ON chat_messages(chat_id) WHERE deleted_at IS NULL;

-- Communities Indexes
CREATE INDEX idx_communities_game_region ON communities(game_focus, region) WHERE privacy = 'public';
CREATE INDEX idx_communities_owner ON communities(owner_id);
CREATE INDEX idx_communities_featured ON communities(featured, member_count DESC) WHERE featured = TRUE;
CREATE INDEX idx_communities_search ON communities USING GIN (to_tsvector('english', name || ' ' || description));
CREATE INDEX idx_chat_messages_system ON chat_messages(receiver_id, message_type, created_at DESC) WHERE message_type = 'system';
CREATE INDEX idx_chat_messages_direct_active ON chat_messages(receiver_id) WHERE deleted_at IS NULL AND receiver_id IS NOT NULL;

-- Community Posts (Social Feed Performance)
CREATE INDEX idx_community_posts_feed ON community_posts(community_id, created_at DESC) WHERE deleted_at IS NULL;
CREATE INDEX idx_community_posts_author ON community_posts(author_id, created_at DESC);
CREATE INDEX idx_community_posts_trending ON community_posts(community_id, upvotes DESC, created_at DESC);

-- Comments (Nested Structure)
CREATE INDEX idx_post_comments_post ON post_comments(post_id, created_at DESC) WHERE deleted_at IS NULL;
CREATE INDEX idx_post_comments_parent ON post_comments(parent_id, created_at ASC) WHERE parent_id IS NOT NULL;

-- Memberships
CREATE INDEX idx_community_members_player ON community_members(player_id, joined_at DESC);
CREATE INDEX idx_community_members_community ON community_members(community_id, role, joined_at DESC);

-- Activity Logs (Audit Performance)
CREATE INDEX idx_activity_logs_entity ON activity_logs(entity_type, entity_id, created_at DESC);
CREATE INDEX idx_activity_logs_actor ON activity_logs(actor_id, created_at DESC);

-- 🔗 ENTERPRISE CONSTRAINTS: Data Integrity
ALTER TABLE teams ADD CONSTRAINT fk_teams_captain FOREIGN KEY (captain) REFERENCES players(id);
ALTER TABLE teams ADD CONSTRAINT fk_teams_organization FOREIGN KEY (organization_id) REFERENCES organizations(id);
ALTER TABLE players ADD CONSTRAINT fk_players_team FOREIGN KEY (team_id) REFERENCES teams(id);
ALTER TABLE organizations ADD CONSTRAINT fk_organizations_approved_by FOREIGN KEY (approved_by) REFERENCES admins(id);

-- Add chat community constraint after communities table exists
ALTER TABLE chats ADD CONSTRAINT fk_chats_community FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE;

-- 🤖 ENTERPRISE AUTOMATION: Auto-cleanup Functions
CREATE OR REPLACE FUNCTION cleanup_expired_sessions()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM user_sessions WHERE expires_at < NOW() - INTERVAL '30 days';
    DELETE FROM rate_limits WHERE window_start < NOW() - INTERVAL '24 hours' AND (blocked_until IS NULL OR blocked_until < NOW());
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_cleanup_auth_data
    AFTER INSERT ON user_sessions
    EXECUTE FUNCTION cleanup_expired_sessions();

-- Auto-update member count
CREATE OR REPLACE FUNCTION update_community_member_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE communities 
        SET member_count = member_count + 1 
        WHERE id = NEW.community_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE communities 
        SET member_count = member_count - 1 
        WHERE id = OLD.community_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_community_member_count
    AFTER INSERT OR DELETE ON community_members
    FOR EACH ROW EXECUTE FUNCTION update_community_member_count();

-- Auto-update comment count
CREATE OR REPLACE FUNCTION update_post_comment_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE community_posts 
        SET comment_count = comment_count + 1 
        WHERE id = NEW.post_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE community_posts 
        SET comment_count = comment_count - 1 
        WHERE id = OLD.post_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_post_comment_count
    AFTER INSERT OR DELETE ON post_comments
    FOR EACH ROW EXECUTE FUNCTION update_post_comment_count();

-- Auto-update timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_chats_updated_at BEFORE UPDATE ON chats FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER trigger_communities_updated_at BEFORE UPDATE ON communities FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER trigger_community_posts_updated_at BEFORE UPDATE ON community_posts FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Foreign Key Constraints
ALTER TABLE chats ADD CONSTRAINT fk_chats_created_by FOREIGN KEY (created_by) REFERENCES players(id);
ALTER TABLE communities ADD CONSTRAINT fk_communities_owner FOREIGN KEY (owner_id) REFERENCES players(id);

-- Data Integrity Constraints
ALTER TABLE chats ADD CONSTRAINT check_chat_participants_valid CHECK (jsonb_typeof(participants) = 'array');
ALTER TABLE communities ADD CONSTRAINT check_member_count_positive CHECK (member_count >= 0);
ALTER TABLE community_posts ADD CONSTRAINT check_vote_counts_positive CHECK (upvotes >= 0 AND downvotes >= 0);

-- 📊 ENTERPRISE MONITORING: Performance Views
CREATE VIEW active_sessions AS
SELECT 
    user_id, user_type, COUNT(*) as session_count,
    MAX(last_activity) as last_seen,
    MIN(created_at) as first_session
FROM user_sessions 
WHERE NOT revoked AND expires_at > NOW()
GROUP BY user_id, user_type;

CREATE VIEW security_metrics AS
SELECT 
    DATE(created_at) as date,
    action,
    COUNT(*) as total_attempts,
    COUNT(*) FILTER (WHERE success) as successful,
    COUNT(*) FILTER (WHERE NOT success) as failed
FROM audit_logs 
WHERE created_at > NOW() - INTERVAL '30 days'
GROUP BY DATE(created_at), action
ORDER BY date DESC, action;

-- Active Chat Statistics
CREATE VIEW chat_activity_stats AS
SELECT 
    DATE(cm.created_at) as date,
    c.chat_type,                    -- Proper normalization
    COUNT(*) as message_count,
    COUNT(DISTINCT cm.sender_id) as active_users,
    COUNT(DISTINCT cm.chat_id) as active_chats
FROM chat_messages cm
JOIN chats c ON cm.chat_id = c.id   -- Enterprise JOIN pattern
WHERE cm.created_at > NOW() - INTERVAL '30 days'
GROUP BY DATE(cm.created_at), c.chat_type
ORDER BY date DESC;

-- Community Engagement Metrics
CREATE VIEW community_engagement AS
SELECT 
    c.id,
    c.name,
    c.member_count,
    COUNT(cp.id) as total_posts,
    COUNT(pc.id) as total_comments,
    SUM(cp.upvotes) as total_upvotes
FROM communities c
LEFT JOIN community_posts cp ON c.id = cp.community_id AND cp.deleted_at IS NULL
LEFT JOIN post_comments pc ON cp.id = pc.post_id AND pc.deleted_at IS NULL
GROUP BY c.id, c.name, c.member_count
ORDER BY c.member_count DESC;

CREATE TABLE player_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    requester_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    recipient_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'accepted', 'declined', 'blocked')),
    message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(requester_id, recipient_id)
);

CREATE INDEX idx_player_connections_recipient ON player_connections(recipient_id, status);
CREATE INDEX idx_player_connections_requester ON player_connections(requester_id, status);

CREATE TABLE recruitment_approaches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    recruiter_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    target_player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'accepted', 'declined', 'withdrawn')),
    message TEXT,
    position_offered VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(team_id, target_player_id, recruiter_id)
);

CREATE INDEX idx_recruitment_approaches_target ON recruitment_approaches(target_player_id, status);
CREATE INDEX idx_recruitment_approaches_recruiter ON recruitment_approaches(recruiter_id, status);
