CREATE DATABASE IF NOT EXISTS memedb;
USE memedb;

CREATE TABLE IF NOT EXISTS memes(
    id INT NOT NULL AUTO_INCREMENT,
    link VARCHAR(150) NOT NULL,
    PRIMARY KEY(id)
);

INSERT INTO memes(link) VALUES
    ("https://cdn.discordapp.com/attachments/820999527083147293/984081669164240936/9pn1egr.jpg"),
	("https://media.discordapp.net/attachments/932770359185387600/986284559761035264/unknown.png");

USE mysql;
CREATE USER 'memebot'@'%' IDENTIFIED BY '907413349e0f87c8ef8551ac4b9848ef';
GRANT ALL PRIVILEGES ON memedb.* TO 'memebot'@'%';
