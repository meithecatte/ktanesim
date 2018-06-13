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

async def cmd_leaderboard(channel, author, parts):
	if not parts:
		page = 1
	elif len(parts) > 1 or not parts[0].isdigit() or parts[0] == '0':
		await channel.send(f"{author.mention} Usage: `{PREFIX}leaderboard [<page number>]`. Default: page 1. Aliases: `{PREFIX}lb`")
		return
	else:
		page = int(parts[0])

	entrycount = conn.execute('SELECT COUNT(1) FROM leaderboard').fetchone()[0]

	if entrycount == 0:
		await channel.send(f"{author.mention} The leaderboard is empty. Change it by solving a module!")
		return

	pagecount = (entrycount - 1) // LEADERBOARD_PAGE_SIZE + 1

	if page > pagecount:
		await channel.send(f"{author.mention} B... but the leaderboard has only {f'{pagecount} pages' if pagecount > 1 else 'a single page'}!")
		return

	offset = (page - 1) * LEADERBOARD_PAGE_SIZE
	entries = conn.execute('SELECT (SELECT COUNT(1) FROM leaderboard AS i WHERE i.points > o.points) + 1 AS position, ' +
		'o.username, o.solves, o.strikes, o.points FROM leaderboard AS o ORDER BY o.points DESC LIMIT ? OFFSET ?', (LEADERBOARD_PAGE_SIZE, offset)).fetchall()
	namewidth = max(map(lambda user: len(user[1]), entries))

	numwidth = len(str(entries[-1][0]))
	reply = f"Leaderboard page {page} of {pagecount}: ```\n{'': <{numwidth + namewidth}}   Solves Strikes Points"
	for index, name, solves, strikes, points in entries:
		reply += f"\n{index: >{numwidth}}. {name: <{namewidth}} {solves: >6} {strikes: >7} {points: >6}"
	reply += "```"
	await channel.send(reply)

async def cmd_rank(channel, author, parts):
	entry = conn.execute('SELECT (SELECT COUNT(1) FROM leaderboard AS i WHERE i.points > o.points) + 1 AS position, ' +
		'o.solves, o.strikes, o.points FROM leaderboard AS o WHERE o.id = ?', (msg.author.id, )).fetchone()
	if entry:
		position, solves, strikes, points = entry
		await channel.send(f"{author.mention} You're #{position} with {solves} solves, {strikes} strikes and {points} points")
	else:
		await channel.send(f"{author.mention} Sorry, you have to play this game to be included in the leaderboard")
