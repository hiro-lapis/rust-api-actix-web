-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('users');

-- WHERE password = crypt('password123', password)
INSERT INTO users (name, email, password) VALUES
('田中太郎', 'tanaka@example.com', crypt('password123', gen_salt('bf'))),
('山田花子', 'yamada@example.com', crypt('password123', gen_salt('bf'))),
('佐藤一郎', 'sato@example.com', crypt('password123', gen_salt('bf'))),
('鈴木美咲', 'suzuki@example.com', crypt('password123', gen_salt('bf'))),
('高橋健太', 'takahashi@example.com', crypt('password123', gen_salt('bf'))),
('伊藤由美', 'ito@example.com', crypt('password123', gen_salt('bf'))),
('渡辺直樹', 'watanabe@example.com', crypt('password123', gen_salt('bf'))),
('小林恵子', 'kobayashi@example.com', crypt('password123', gen_salt('bf'))),
('加藤誠', 'kato@example.com', crypt('password123', gen_salt('bf'))),
('吉田あゆみ', 'yoshida@example.com', crypt('password123', gen_salt('bf')));
