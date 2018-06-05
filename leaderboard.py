import sqlite3
ver = list(map(int, sqlite3.sqlite_version.split('.')))
assert ver[0] > 3 or ver[0] == 3 and ver[1] >= 24, "Update your sqlite!"
del ver

conn = sqlite3.connect('leaderboard.dat')
conn.execute('CREATE TABLE IF NOT EXISTS leaderboard (id integer PRIMARY KEY, username text, points integer, solves integer, strikes integer)')
conn.commit()

def get_top(offset, count):
	return conn.execute('SELECT * FROM leaderboard ORDER BY points DESC LIMIT ? OFFSET ?', (count, offset))

def _record(user, delta):
	conn.execute('INSERT INTO leaderboard (id, username, points, solves, strikes) VALUES (?, ?, ?, ?, ?) ' +
		'ON CONFLICT(id) DO UPDATE SET username = ?, points = points + ?, solves = solves + ?, strikes = strikes + ?',
		(user.id, str(user), *delta, str(user), *delta))
	conn.commit()

def record_solve(user, weight):
	_record(user, (weight, 1, 0))

def record_strike(user, weight):
	_record(user, (-weight, 0, 1))

def record_penalty(user, points):
	_record(user, (-weight, 0, 0))
