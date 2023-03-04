INSERT INTO teams (rank, score, stage, name, description, creation_date, location) VALUES ( 1, 800, 1, 'Startup-tec', 'TEC startup club', '2023-02-26', 'Monterrey, Mexico');
INSERT INTO teams (rank, score, stage, name, description, creation_date, location) VALUES ( 2, 800, 1, 'Moneypool', 'TEC startup club', '2023-02-26', 'Monterrey, Mexico');

INSERT INTO labels (name) VALUES ('Web3');
INSERT INTO labels (name) VALUES ('Edtec');
INSERT INTO labels (name) VALUES ('Videogame');
INSERT INTO labels (name) VALUES ('Marketplace');

INSERT INTO badge_categories (name) VALUES ('Talk');
INSERT INTO badge_categories (name) VALUES ('Stage 1');
INSERT INTO badge_categories (name) VALUES ('Stage 2');

INSERT INTO badges (name, description, points, category) VALUES ('Logro 1', 'Primer logro del viaje', 200, 2);
INSERT INTO badges (name, description, points, category) VALUES ('Platica de Felipe Ivan', 'Platica sobre como empezar un startup dada por Felipe Ivan', 700, 1);

INSERT INTO persons (team_id, name, career, graduation_date, picture_url) VALUES (1, 'Felipe Ivan', 'ITC', '2026-01-01', 'https:');
INSERT INTO persons (team_id, name, career, graduation_date, picture_url) VALUES (2, 'Nacho', 'BGB', '2023-01-01', 'https:');

INSERT INTO label_ownerships(team_id, label_id) VALUES (1, 2);
INSERT INTO label_ownerships(team_id, label_id) VALUES (1, 3);
INSERT INTO label_ownerships(team_id, label_id) VALUES (2, 2);
INSERT INTO label_ownerships(team_id, label_id) VALUES (2, 1);

INSERT INTO badge_ownerships(team_id, badge_id, acquisition_date) VALUES (1, 1, '2023-01-23');
INSERT INTO badge_ownerships(team_id, badge_id, acquisition_date) VALUES (1, 2, '2023-01-23');
INSERT INTO badge_ownerships(team_id, badge_id, acquisition_date) VALUES (2, 1, '2023-01-23');
INSERT INTO badge_ownerships(team_id, badge_id, acquisition_date) VALUES (2, 2, '2023-01-23');