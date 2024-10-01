DROP TABLE IF EXISTS Guests;
CREATE TABLE IF NOT EXISTS Guests (username TEXT PRIMARY KEY, fullname TEXT, title TEXT, address TEXT, session TEXT);
INSERT INTO Guests (username, fullname, title, address, session) VALUES ('andi', 'ANDI', 'Jabatan', 'INDONESIA', '11.00 - 12.00 WIB'), ('budi', 'Budi', '', '', '11.00 - 12.00 WIB'), ('susi', 'Susi', '', 'INDONESIA', '12.00 - 13.00 WIB');