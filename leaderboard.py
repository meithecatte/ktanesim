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
	entrycount = next(conn.execute('SELECT COUNT(1) FROM leaderboard'))[0]

	if entrycount == 0:
		return "The leaderboard is empty. Change it by solving a module!"

	pagecount = (entrycount - 1) // pagesize + 1

	if page > pagecount:
		return "B... but the leaderboard has only " + ("{:d} pages".format(pagecount) if pagecount > 1 else "one page")

	offset = (page - 1) * pagesize
	entries = conn.execute('SELECT username, solves, strikes, points FROM leaderboard ORDER BY points DESC LIMIT ? OFFSET ?', (pagesize, offset)).fetchall()
	max_name_length = max(map(lambda user: len(user[0]), entries))

	index = (page - 1) * pagesize + 1
	last_index = index + len(entries) - 1
	max_index_digits = len(str(last_index))
	reply = "Leaderboard page {:d} of {:d}: ```\n{:s}   Solves Strikes Points".format(page, pagecount, ' ' * (max_index_digits + max_name_length))
	for entry in entries:
		reply += "\n%s. %s %6d %7d %6d" % (str(index).rjust(max_index_digits), entry[0].ljust(max_name_length), entry[1], entry[2], entry[3])
		index += 1
	reply += "```"
	return reply
