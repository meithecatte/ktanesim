import sqlite3
from config import LEADERBOARD_PAGE_SIZE
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

async def cmd_leaderboard(msg, parts):
	if not parts:
		page = 1
	elif len(parts) > 1 or not parts[0].isdigit() or parts[0] == '0':
		await msg.channel.send("{:s} Usage: `{prefix}leaderboard [<page number>]`. Default: page 1. Aliases: `{prefix}lb`".format(msg.author.mention, prefix=PREFIX))
		return
	else:
		page = int(parts[0])

	entrycount = conn.execute('SELECT COUNT(1) FROM leaderboard').fetchone()[0]

	if entrycount == 0:
		await msg.channel.send("{:s} The leaderboard is empty. Change it by solving a module!".format(msg.author.mention))
		return

	pagecount = (entrycount - 1) // LEADERBOARD_PAGE_SIZE + 1

	if page > pagecount:
		await msg.channel.send("{:s} B... but the leaderboard has only ".format(msg.author.mention) + ("{:d} pages!".format(pagecount) if pagecount > 1 else "one page!"))
		return

	offset = (page - 1) * LEADERBOARD_PAGE_SIZE
	entries = conn.execute('SELECT (SELECT COUNT(1) FROM leaderboard AS i WHERE i.points > o.points) + 1 AS position, ' +
		'o.username, o.solves, o.strikes, o.points FROM leaderboard AS o ORDER BY o.points DESC LIMIT ? OFFSET ?', (LEADERBOARD_PAGE_SIZE, offset)).fetchall()
	max_name_length = max(map(lambda user: len(user[1]), entries))

	max_index_digits = len(str(entries[-1][0]))
	reply = "Leaderboard page {:d} of {:d}: ```\n{:s}   Solves Strikes Points".format(page, pagecount, ' ' * (max_index_digits + max_name_length))
	for entry in entries:
		reply += "\n{:-{numwidth}d}. {:{namewidth}s} {:6d} {:7d} {:6d}".format(*entry, numwidth=max_index_digits, namewidth=max_name_length)
	reply += "```"
	await msg.channel.send(reply)

async def cmd_rank(msg, parts):
	entry = conn.execute('SELECT (SELECT COUNT(1) FROM leaderboard AS i WHERE i.points > o.points) + 1 AS position, ' +
		'o.solves, o.strikes, o.points FROM leaderboard AS o WHERE o.id = ?', (msg.author.id, )).fetchone()
	if entry:
		await msg.channel.send("{:s} You're #{:d} with {:d} solves, {:d} strikes and {:d} points".format(user.mention, *entry))
	else:
		await msg.channel.send("{:s} Sorry, you have to play this game to be included in the leaderboard".format(user.mention))
