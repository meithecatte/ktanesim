import sqlite3
ver = list(map(int, sqlite3.sqlite_version.split('.')))
assert ver[0] > 3 or ver[0] == 3 and ver[1] >= 24, "Update your sqlite!"
del ver

conn = sqlite3.connect('leaderboard.dat')
conn.execute('CREATE TABLE IF NOT EXISTS leaderboard (id integer PRIMARY KEY, username text, points integer, solves integer, strikes integer)')
conn.commit()

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
	_record(user, (-points, 0, 0))

def format_page(page, pagesize):
	entrycount = conn.execute('SELECT COUNT(1) FROM leaderboard').fetchone()[0]

	if entrycount == 0:
		return "The leaderboard is empty. Change it by solving a module!"

	pagecount = (entrycount - 1) // pagesize + 1

	if page > pagecount:
		return "B... but the leaderboard has only " + ("{:d} pages".format(pagecount) if pagecount > 1 else "one page")

	offset = (page - 1) * pagesize
	entries = conn.execute('SELECT (SELECT COUNT(1) FROM leaderboard AS i WHERE i.points > o.points) + 1 AS position, o.username, o.solves, o.strikes, o.points FROM leaderboard AS o ORDER BY o.points DESC LIMIT ? OFFSET ?', (pagesize, offset)).fetchall()
	max_name_length = max(map(lambda user: len(user[1]), entries))

	max_index_digits = len(str(entries[-1][0]))
	reply = "Leaderboard page {:d} of {:d}: ```\n{:s}   Solves Strikes Points".format(page, pagecount, ' ' * (max_index_digits + max_name_length))
	for entry in entries:
		reply += "\n{:-{numwidth}d}. {:{namewidth}s} {:6d} {:7d} {:6d}".format(*entry, numwidth=max_index_digits, namewidth=max_name_length)
	reply += "```"
	return reply

def format_rank(user):
	entry = conn.execute('SELECT (SELECT COUNT(1) FROM leaderboard AS i WHERE i.points > o.points) + 1 AS position, o.solves, o.strikes, o.points FROM leaderboard AS o WHERE o.id = ?', (user.id, )).fetchone()
	if entry:
		return "{:s} You're #{:d} with {:d} solves, {:d} strikes and {:d} points".format(user.mention, *entry)
	else:
		return "{:s} Sorry, you have to play this game to be included in the leaderboard".format(user.mention)
