-- Database

DROP DATABASE IF EXISTS basketball;

CREATE DATABASE IF NOT EXISTS basketball;

-- Types

CREATE TYPE basketball.position AS ENUM ('PG', 'SG', 'SF', 'PF', 'C');

CREATE TYPE basketball.main_hand AS ENUM ('LEFT', 'RIGHT', 'AMBIDEXTROUS');

-- Tables

CREATE TABLE IF NOT EXISTS basketball.teams
(
    id         UUID        NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    name       VARCHAR(63) NOT NULL,
    created_at TIMESTAMP   NOT NULL        DEFAULT now(),
    updated_at TIMESTAMP                   DEFAULT NULL,
    deleted_at TIMESTAMP                   DEFAULT NULL,
    is_deleted BOOLEAN     NOT NULL        DEFAULT FALSE,
    PRIMARY KEY (id),
    INDEX (name)
);

CREATE TABLE IF NOT EXISTS basketball.identities
(
    id            UUID        NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    first_name    VARCHAR(63) NOT NULL,
    last_name     VARCHAR(63) NOT NULL,
    email         VARCHAR(127)                DEFAULT NULL,
    phone         VARCHAR(10)                 DEFAULT NULL,
    birth_date    DATE                        DEFAULT NULL,
    social_number VARCHAR(63) UNIQUE          DEFAULT NULL,
    created_at    TIMESTAMP   NOT NULL        DEFAULT now(),
    updated_at    TIMESTAMP                   DEFAULT NULL,
    deleted_at    TIMESTAMP                   DEFAULT NULL,
    is_deleted    BOOLEAN     NOT NULL        DEFAULT FALSE,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS basketball.users
(
    id                 UUID         NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    identity_id        UUID UNIQUE                  DEFAULT NULL,
    username           VARCHAR(63)  NOT NULL UNIQUE,
    recovery_email     VARCHAR(127)                 DEFAULT NULL,
    password           VARCHAR(255) NOT NULL,
    refresh_token      VARCHAR(255)                 DEFAULT NULL,
    verification_token VARCHAR(255) NOT NULL,
    logged_at          TIMESTAMP                    DEFAULT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (identity_id) REFERENCES basketball.identities (id) ON DELETE CASCADE,
    INDEX (identity_id, username)
);

CREATE TABLE IF NOT EXISTS basketball.players
(
    id          UUID                 NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    identity_id UUID                 NOT NULL UNIQUE,
    team_id     UUID                 NOT NULL,
    nickname    VARCHAR(63)                          DEFAULT NULL,
    number      INT                  NOT NULL CHECK (number BETWEEN 0 AND 99),
    position    basketball.position  NOT NULL,
    height      DECIMAL(5, 2)                        DEFAULT NULL,
    weight      DECIMAL(5, 2)                        DEFAULT NULL,
    wingspan    DECIMAL(5, 2)                        DEFAULT NULL,
    main_hand   basketball.main_hand NOT NULL        DEFAULT 'RIGHT',
    PRIMARY KEY (id),
    FOREIGN KEY (identity_id) REFERENCES basketball.identities (id) ON DELETE CASCADE,
    FOREIGN KEY (team_id) REFERENCES basketball.teams (id) ON DELETE CASCADE,
    INDEX (identity_id, team_id)
);

-- Grants

CREATE USER IF NOT EXISTS basketball WITH PASSWORD NULL;

ALTER DATABASE basketball OWNER TO basketball;

GRANT ALL ON DATABASE basketball TO basketball;

GRANT ALL ON TABLE basketball.* TO basketball;

-- Seeding TODO

INSERT INTO basketball.identities (first_name, last_name, email, birth_date)
VALUES
    ('LeBron', 'James', 'lebron.james@nba.com', '1984-12-30'),
    ('Stephen', 'Curry', 'stephen.curry@nba.com', '1988-03-14'),
    ('Kevin', 'Durant', 'kevin.durant@nba.com', '1988-09-29'),
    ('Kyrie', 'Irving', 'kyrie.irving@nba.com', '1992-03-23'),
    ('James', 'Harden', 'james.harden@nba.com', '1989-08-26'),
    ('Giannis', 'Antetokounmpo', 'giannis.antetokounmpo@nba.com', '1994-12-06'),
    ('Kawhi', 'Leonard', 'kawhi.leonard@nba.com', '1991-06-29'),
    ('Russell', 'Westbrook', 'russell.westbrook@nba.com', '1988-11-12'),
    ('Paul', 'George', 'paul.george@nba.com', '1990-05-02'),
    ('Anthony', 'Davis', 'anthony.davis@nba.com', '1993-03-11'),
    ('Damian', 'Lillard', 'damian.lillard@nba.com', '1990-07-15'),
    ('Jimmy', 'Butler', 'jimmy.butler@nba.com', '1989-09-14'),
    ('Chris', 'Paul', 'chris.paul@nba.com', '1985-05-06'),
    ('Devin', 'Booker', 'devin.booker@nba.com', '1996-10-30'),
    ('Zion', 'Williamson', 'zion.williamson@nba.com', '2000-07-06'),
    ('Jayson', 'Tatum', 'jayson.tatum@nba.com', '1998-03-03'),
    ('Trae', 'Young', 'trae.young@nba.com', '1998-09-19'),
    ('Nikola', 'Jokic', 'nikola.jokic@nba.com', '1995-02-19'),
    ('Luka', 'Doncic', 'luka.doncic@nba.com', '1999-02-28'),
    ('Victor', 'Oladipo', 'victor.oladipo@nba.com', '1992-05-01'),
    ('Ben', 'Simmons', 'ben.simmons@nba.com', '1996-07-20'),
    ('Karl-Anthony', 'Towns', 'karl-anthony.towns@nba.com', '1995-11-15'),
    ('Rudy', 'Gobert', 'rudy.gobert@nba.com', '1992-06-26'),
    ('Donovan', 'Mitchell', 'donovan.mitchell@nba.com', '1996-09-07'),
    ('Julius', 'Randle', 'julius.randle@nba.com', '1994-11-29'),
    ('Bradley', 'Beal', 'bradley.beal@nba.com', '1993-06-28'),
    ('Draymond', 'Green', 'draymond.green@nba.com', '1990-03-04'),
    ('Kemba', 'Walker', 'kemba.walker@nba.com', '1990-05-08'),
    ('DeMar', 'DeRozan', 'demar.derozan@nba.com', '1989-08-07'),
    ('Serge', 'Ibaka', 'serge.ibaka@nba.com', '1989-09-18'),
    ('Clint', 'Capela', 'clint.capela@nba.com', '1994-05-18'),
    ('C.J.', 'McCollum', 'cj.mccollum@nba.com', '1991-09-19'),
    ('Jrue', 'Holiday', 'jrue.holiday@nba.com', '1990-06-12'),
    ('Al', 'Horford', 'al.horford@nba.com', '1986-06-03'),
    ('Tobias', 'Harris', 'tobias.harris@nba.com', '1992-07-15'),
    ('Marcus', 'Smart', 'marcus.smart@nba.com', '1994-03-06'),
    ('Kevin', 'Love', 'kevin.love@nba.com', '1988-09-07'),
    ('Gordon', 'Hayward', 'gordon.hayward@nba.com', '1990-03-23'),
    ('Michael', 'Porter Jr.', 'michael.porter.jr@nba.com', '1998-06-26'),
    ('Jaren', 'Jackson Jr.', 'jaren.jackson.jr@nba.com', '1999-09-15'),
    ('Bam', 'Adebayo', 'bam.adebayo@nba.com', '1997-07-18'),
    ('Jonas', 'Valanciunas', 'jonas.valanciunas@nba.com', '1992-05-06'),
    ('Derrick', 'Rose', 'derrick.rose@nba.com', '1988-10-04'),
    ('Chris', 'Bosh', 'chris.bosh@nba.com', '1984-03-24'),
    ('Pau', 'Gasol', 'pau.gasol@nba.com', '1980-07-06'),
    ('Kevin', 'Garnett', 'kevin.garnett@nba.com', '1976-05-19'),
    ('Ray', 'Allen', 'ray.allen@nba.com', '1975-07-20'),
    ('Kobe', 'Bryant', 'kobe.bryant@nba.com', '1978-08-23'),
    ('Tim', 'Duncan', 'tim.duncan@nba.com', '1976-04-25'),
    ('Allen', 'Iverson', 'allen.iverson@nba.com', '1975-06-07'),
    ('Reggie', 'Miller', 'reggie.miller@nba.com', '1965-08-24'),
    ('Magic', 'Johnson', 'magic.johnson@nba.com', '1959-08-14'),
    ('Larry', 'Bird', 'larry.bird@nba.com', '1956-12-07'),
    ('Shaquille', 'Neal', 'shaquille.oneal@nba.com', '1972-03-06'),
    ('Bill', 'Russell', 'bill.russell@nba.com', '1934-02-12'),
    ('Hakeem', 'Olajuwon', 'hakeem.olajuwon@nba.com', '1963-01-21'),
    ('Oscar', 'Robertson', 'oscar.robertson@nba.com', '1938-11-24'),
    ('Wilt', 'Chamberlain', 'wilt.chamberlain@nba.com', '1936-08-21'),
    ('Kareem', 'Abdul-Jabbar', 'kareem.abdul-jabbar@nba.com', '1947-04-16'),
    ('George', 'Mikan', 'george.mikan@nba.com', '1924-06-18'),
    ('John', 'Havlicek', 'john.havlicek@nba.com', '1940-04-08'),
    ('Chris', 'Webber', 'chris.webber@nba.com', '1973-03-01'),
    ('Kevin', 'McHale', 'kevin.mchale@nba.com', '1957-12-19'),
    ('Charles', 'Barkley', 'charles.barkley@nba.com', '1963-02-20'),
    ('Scottie', 'Pippen', 'scottie.pippen@nba.com', '1965-09-25'),
    ('Dennis', 'Rodman', 'dennis.rodman@nba.com', '1961-05-13'),
    ('Steve', 'Nash', 'steve.nash@nba.com', '1974-02-07'),
    ('Isiah', 'Thomas', 'isiah.thomas@nba.com', '1961-04-30'),
    ('Alonzo', 'Mourning', 'alonzo.mourning@nba.com', '1970-02-08'),
    ('David', 'Robinson', 'david.robinson@nba.com', '1965-08-06'),
    ('Kevin', 'Johnson', 'kevin.johnson@nba.com', '1966-03-04'),
    ('Dikembe', 'Mutombo', 'dikembe.mutombo@nba.com', '1966-06-25'),
    ('Yao', 'Ming', 'yao.ming@nba.com', '1980-09-12');

INSERT INTO basketball.users (identity_id, username, password, verification_token)
VALUES
    ((SELECT id FROM basketball.identities WHERE email = 'lebron.james@nba.com'), 'lebron.james', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'stephen.curry@nba.com'), 'stephen.curry', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'kevin.durant@nba.com'), 'kevin.durant', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'kyrie.irving@nba.com'), 'kyrie.irving', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'james.harden@nba.com'), 'james.harden', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'giannis.antetokounmpo@nba.com'), 'giannis.antetokounmpo', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'kawhi.leonard@nba.com'), 'kawhi.leonard', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'russell.westbrook@nba.com'), 'russell.westbrook', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'paul.george@nba.com'), 'paul.george', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'anthony.davis@nba.com'), 'anthony.davis', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'damian.lillard@nba.com'), 'damian.lillard', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'jimmy.butler@nba.com'), 'jimmy.butler', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'chris.paul@nba.com'), 'chris.paul', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'devin.booker@nba.com'), 'devin.booker', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'zion.williamson@nba.com'), 'zion.williamson', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'jayson.tatum@nba.com'), 'jayson.tatum', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'trae.young@nba.com'), 'trae.young', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'nikola.jokic@nba.com'), 'nikola.jokic', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'luka.doncic@nba.com'), 'luka.doncic', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'victor.oladipo@nba.com'), 'victor.oladipo', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'ben.simmons@nba.com'), 'ben.simmons', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'karl-anthony.towns@nba.com'), 'karl-anthony.towns', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'rudy.gobert@nba.com'), 'rudy.gobert', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'donovan.mitchell@nba.com'), 'donovan.mitchell', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'julius.randle@nba.com'), 'julius.randle', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'bradley.beal@nba.com'), 'bradley.beal', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'draymond.green@nba.com'), 'draymond.green', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'kemba.walker@nba.com'), 'kemba.walker', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'demar.derozan@nba.com'), 'demar.derozan', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'serge.ibaka@nba.com'), 'serge.ibaka', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'clint.capela@nba.com'), 'clint.capela', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'cj.mccollum@nba.com'), 'cj.mccollum', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'jrue.holiday@nba.com'), 'jrue.holiday', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'al.horford@nba.com'), 'al.horford', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'tobias.harris@nba.com'), 'tobias.harris', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'marcus.smart@nba.com'), 'marcus.smart', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'kevin.love@nba.com'), 'kevin.love', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'gordon.hayward@nba.com'), 'gordon.hayward', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'michael.porter.jr@nba.com'), 'michael.porter.jr', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'jaren.jackson.jr@nba.com'), 'jaren.jackson.jr', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'bam.adebayo@nba.com'), 'bam.adebayo', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'jonas.valanciunas@nba.com'), 'jonas.valanciunas', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'derrick.rose@nba.com'), 'derrick.rose', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'chris.bosh@nba.com'), 'chris.bosh', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'pau.gasol@nba.com'), 'pau.gasol', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'kevin.garnett@nba.com'), 'kevin.garnett', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'ray.allen@nba.com'), 'ray.allen', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'kobe.bryant@nba.com'), 'kobe.bryant', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'tim.duncan@nba.com'), 'tim.duncan', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'allen.iverson@nba.com'), 'allen.iverson', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'reggie.miller@nba.com'), 'reggie.miller', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'magic.johnson@nba.com'), 'magic.johnson', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'larry.bird@nba.com'), 'larry.bird', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'shaquille.oneal@nba.com'), 'shaquille.oneal', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'bill.russell@nba.com'), 'bill.russell', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'hakeem.olajuwon@nba.com'), 'hakeem.olajuwon', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'oscar.robertson@nba.com'), 'oscar.robertson', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'wilt.chamberlain@nba.com'), 'wilt.chamberlain', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'kareem.abdul-jabbar@nba.com'), 'kareem.abdul-jabbar', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'george.mikan@nba.com'), 'george.mikan', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'john.havlicek@nba.com'), 'john.havlicek', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'chris.webber@nba.com'), 'chris.webber', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'kevin.mchale@nba.com'), 'kevin.mchale', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'charles.barkley@nba.com'), 'charles.barkley', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'scottie.pippen@nba.com'), 'scottie.pippen', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'dennis.rodman@nba.com'), 'dennis.rodman', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'steve.nash@nba.com'), 'steve.nash', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'isiah.thomas@nba.com'), 'isiah.thomas', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'alonzo.mourning@nba.com'), 'alonzo.mourning', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'david.robinson@nba.com'), 'david.robinson', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'kevin.johnson@nba.com'), 'kevin.johnson', 'hashed_password', 'verification_token'),
    ((SELECT id FROM basketball.identities WHERE email = 'dikembe.mutombo@nba.com'), 'dikembe.mutombo', 'hashed_password', 'verification_token');