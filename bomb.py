import sys
import time
import random
import asyncio
import aiohttp
import discord
import modules
import traceback
from config import *

class BatteryWidget:
	def __init__(self):
		self.battery_count = random.randint(1, 2)

class IndicatorWidget:
	INDICATORS = ['SND', 'CLR', 'CAR', 'IND', 'FRQ', 'SIG', 'NSA', 'MSA', 'TRN', 'BOB', 'FRK']

	def __init__(self):
		self.code = random.choice(self.INDICATORS)
		self.lit = random.random() > 0.4

	def __str__(self):
		return ('*' if self.lit else '') + self.code

class PortPlateWidget:
	PORT_GROUPS = [['Serial', 'Parallel'], ['DVI', 'PS2', 'RJ45', 'StereoRCA']]

	def __init__(self):
		group = random.choice(self.PORT_GROUPS)
		self.ports = []
		for port in group:
			if random.random() > 0.5:
				self.ports.append(port)

	def __str__(self):
		return '[' + (', '.join(self.ports) if self.ports else 'Empty') + ']'

class Bomb:
	SERIAL_NUMBER_CHARACTERS = "ABCDEFGHIJKLMNEPQRSTUVWXZ0123456789"
	EDGEWORK_WIDGETS = [BatteryWidget, IndicatorWidget, PortPlateWidget]
	bombs = {}
	hastebin_session = None
	client = None
	shutdown_mode = False

	def __init__(self, channel, modules, hummus = False):
		self.channel = channel
		self.hummus = hummus
		self.strikes = 0
		self.start_time = time.monotonic()
		self.serial = self._randomize_serial()

		self.edgework = []
		for _ in range(5):
			self.edgework.append(random.choice(Bomb.EDGEWORK_WIDGETS)())

		self.modules = []
		for index, module in enumerate(modules):
			self.modules.append(module(self, index + 1))

	@staticmethod
	async def update_presence():
		await Bomb.client.change_presence(activity=discord.Game(f"{len(Bomb.bombs)} bomb{'s' if len(Bomb.bombs) != 1 else ''}. {PREFIX}help for help"))

	@staticmethod
	async def cmd_shutdown(channel, author, parts):
		if parts:
			await channel.send(f"{author.mention} Trailing arguments.")
			return

		if author.id != BOT_OWNER:
			await channel.send(f"{author.mention} You don't have permission to use this command.")
			return

		Bomb.shutdown_mode = True

		for bomb_channel in Bomb.bombs:
			asyncio.ensure_future(bomb_channel.send(f"The bot is going into shutdown mode. No new bombs can be started."))

		if not Bomb.bombs:
			await channel.send(f"***oof***")
			Bomb.client.loop.stop()
		else:
			await channel.send(f"{author.mention} Shutdown mode activated")

	@staticmethod
	async def cmd_run(channel, author, parts):
		if channel in Bomb.bombs:
			await channel.send(f"{author.mention} A bomb is already ticking in this channel!")
			return

		if Bomb.shutdown_mode:
			await channel.send(f"{author.mention} The bot is in shutdown mode. No new bombs can be started.")
			return

		usage = (
			f"{author.mention} Usage: `{PREFIX}run [hummus] <module count> <module distributon> [-<module 1> [-<module 2> [...]]]` or "
			f"`{PREFIX}run [hummus] <module 1> [<module 2> [...]]`.\n"
			f"For example:\n - `{PREFIX}run hummus 7 vanilla` - 7 vanilla modules that use the modified manual by LtHummus\n"
			f" - `{PREFIX}run 12 mixed -souvenir -theCube` - 12 modules, half of which being vanilla. "
			f"Souvenir and The Cube modules will not be generated\n"
			f" - `{PREFIX}run marbleTumble` - a single Marble Tumble module and nothing else\n"
			f" - `{PREFIX}run hummus complicatedWires morseCode 3Dmaze` - three modules: Complicated Wires and Morse Code, both using LtHummus's manual,"
			f" and 3D Maze, using the normal manual since only vanilla modules support hummus."
			f"Use `{PREFIX}modules` to see the implemented modules.\nAvailable distributions:")

		distributions = {
			"vanilla": 1,
			"mods": 0,
			"modded": 0,
			"mixed": 0.5,
			"lightmixed": 0.67,
			"mixedlight": 0.67,
			"heavymixed": 0.33,
			"mixedheavy": 0.33,
			"light": 0.8,
			"heavy": 0.2,
			"extralight": 0.9,
			"extraheavy": 0.1
		}

		for distribution in distributions:
			if distribution not in ["lightmixed", "heavymixed", "modded"]:
				vanilla = int(distributions[distribution] * 100)
				usage += f"\n`{distribution}`: {vanilla}% vanilla, {100 - vanilla}% modded"

		if len(parts) < 1:
			return await channel.send(usage.format(author.mention, prefix=PREFIX))

		hummus = parts[0].lower() == "hummus"
		if hummus: parts.pop(0)

		if parts[0].isdigit():
			if len(parts) < 2 or parts[1].lower() not in distributions:
				await channel.send(usage.format(author.mention, prefix=PREFIX))
				return

			module_candidates_vanilla = modules.VANILLA_MODULES.copy()
			module_candidates_modded = modules.MODDED_MODULES.copy()
			module_count = int(parts[0])

			if module_count == 0:
				return await channel.send(f"{author.mention} What would it even mean for a bomb to have no modules? :thinking:")

			for veto in parts[2:]:
				if not veto.startswith('-'):
					await channel.send(usage)
					return

				veto = veto[1:]

				if veto in module_candidates_vanilla:
					del module_candidates_vanilla[veto]
				elif veto in module_candidates_modded:
					del module_candidates_modded[veto]
				else:
					return await channel.send(f"{author.mention} No such module: `{veto}`")

			chosen_modules = []

			vanilla_count = distributions[parts[1].lower() ]* module_count

			if (not module_candidates_vanilla or vanilla_count == 0) and (not module_candidates_modded or vanilla_count == module_count):
				return await channel.send(f"{author.mention} You've blacklisted all the modules! If you don't want to play, just say so!")

			if not module_candidates_vanilla: vanilla_count = 0
			elif not module_candidates_modded: vanilla_count = module_count

			for _ in range(vanilla_count):
				chosen_modules.append(random.choice(list(module_candidates_vanilla.values())))

			for _ in range(module_count - vanilla_count):
				chosen_modules.append(random.choice(list(module_candidates_modded.values())))
		else:
			chosen_modules = []
			module_candidates = {**modules.VANILLA_MODULES, **modules.MODDED_MODULES}
			for module in parts:
				if '*' in module:
					if module.count('*') > 1:
						return await channel.send(f"{author.mention} Don't you think there's too many stars in `{module}`?")
					left, right = module.split('*')
					if left.isdigit() and not right.isdigit():
						count = int(left)
						module = right
					elif not left.isdigit() and right.isdigit():
						count = int(right)
						module = left
					else:
						return await channel.send(f"{author.mention} `{module}`: which one is the module and which one is the count?")
				else:
					count = 1
				if module not in module_candidates:
					return await channel.send(f"{author.mention} No such module: `{module}`")
				chosen_modules.extend([module_candidates[module]] * count)

		bomb = Bomb(channel, chosen_modules, hummus)
		Bomb.bombs[channel] = bomb
		await channel.send(f"A bomb with {len(bomb.modules)} modules has been armed!\nEdgework: `{bomb.get_edgework()}`")
		await Bomb.update_presence()

	async def bomb_end(self, boom=False):
		if Bomb.hastebin_session is None:
			Bomb.hastebin_session = aiohttp.ClientSession(timeout=aiohttp.ClientTimeout(total=5))

		try:
			async with self.hastebin_session.post('https://hastebin.com/documents', data=self.get_log().encode('utf-8')) as resp:
				decoded = await resp.json()
				if 'key' in decoded:
					logurl = f"Log: https://hastebin.com/{decoded['key']}.txt"
				elif 'message' in decoded:
					logurl = f"Log upload failed with error message: `{decoded['message']}`"
				else:
					logurl = f"Log upload failed with no error message: `{repr(decoded)}`"
		except Exception:
			logurl = f"Log upload failed with exception: ```\n{traceback.format_exc()}```"
		await self.channel.send(f"{':boom:' if boom else ''} The bomb has been {'**detonated**' if boom else 'defused'} after {self.get_time_formatted()} and {self.strikes} strike{'s' if self.strikes != 1 else ''}. {logurl}")
		del Bomb.bombs[self.channel]
		if Bomb.shutdown_mode and not Bomb.bombs:
			Bomb.client.loop.stop()
		else:
			await Bomb.update_presence()

	def get_log(self):
		log = ["Edgework: {:s}".format(self.get_edgework())]
		for module in self.modules:
			log.append(module.get_log())
		return '\n\n'.join(log)

	def get_claims(self, user):
		return [module for module in self.modules if not module.solved and module.claim is not None and module.claim.id == user.id]

	def get_widgets(self, type_):
		return list(filter(lambda widget: type(widget) is type_, self.edgework))

	def get_battery_count(self):
		return sum(widget.battery_count for widget in self.get_widgets(BatteryWidget))

	def get_holder_count(self):
		return len(self.get_widgets(BatteryWidget))

	def has_lit_indicator(self, code):
		for indicator in self.get_widgets(IndicatorWidget):
			if indicator.lit and indicator.code == code:
				return True

	def has_vowel(self):
		for vowel in "AEIOU":
			if vowel in self.serial:
				return True

		return False

	def get_edgework(self):
		edgework = [
			'{:d}B {:d}H'.format(self.get_battery_count(), self.get_holder_count()),
			' '.join(map(str, self.get_widgets(IndicatorWidget))),
			' '.join(map(str, self.get_widgets(PortPlateWidget))),
			self.serial]
		return ' // '.join(widget for widget in edgework if widget != '')

	def get_unclaimed(self):
		return [module for module in self.modules if module.claim is None and not module.solved]

	def get_time(self):
		return time.monotonic() - self.start_time

	def get_time_formatted(self):
		seconds = int(self.get_time())
		minutes = seconds // 60
		seconds %= 60
		hours = minutes // 60
		minutes %= 60
		return '{:d}:{:02d}:{:02d}'.format(hours, minutes, seconds)

	def get_solved_count(self):
		return sum(module.solved for module in self.modules)

	def _randomize_serial(self):
		def get_any():
			return random.choice(Bomb.SERIAL_NUMBER_CHARACTERS)

		def get_letter():
			return random.choice(Bomb.SERIAL_NUMBER_CHARACTERS[:-10])

		def get_digit():
			return str(random.randint(0, 9))

		return get_any() + get_any() + get_digit() + get_letter() + get_letter() + get_digit()

	async def handle_command(self, command, author, parts):
		if command in Bomb.COMMANDS:
			await Bomb.COMMANDS[command](self, author, parts)
		elif command.isdigit():
			ident = int(command)
			if ident not in range(1, len(self.modules) + 1):
				await self.channel.send("f{author.mention} Double check the module number - there are only {len(self.modules)} modules on this bomb!")
			elif not parts:
				await self.channel.send("f{author.mention} What should I do with module {ident}? You need to give me a command!")
			else:
				command = parts.pop(0).lower()
				await self.modules[ident - 1].handle_command(command, author, parts)

	async def cmd_edgework(self, author, parts):
		await self.channel.send(f"{author.mention} Edgework: `{self.get_edgework()}`")

	async def cmd_unclaimed(self, author, parts):
		unclaimed = self.get_unclaimed()

		if unclaimed:
			if len(unclaimed) > MAX_UNCLAIMED_LIST_SIZE:
				reply = f'{MAX_UNCLAIMED_LIST_SIZE} randomly chosen unclaimed modules:'
				unclaimed = random.sample(unclaimed, MAX_UNCLAIMED_LIST_SIZE)
				unclaimed.sort(key=lambda module: module.ident)
			else:
				reply = 'Unclaimed modules:'

			for module in unclaimed:
				reply += f"\n#{module.ident}: {module.display_name}"
		else:
			reply = "There are no unclaimed modules."

		await self.channel.send(reply)

	async def cmd_claims(self, author, parts):
		claims = list(map(str, self.get_claims(author)))
		if len(claims) == 0:
			await self.channel.send(f"{author.mention} You have not claimed any modules.")
		elif len(claims) == 1:
			await self.channel.send(f"{author.mention} You have only claimed {claims[0]}.")
		else:
			await self.channel.send(f"{author.mention} You have claimed {', '.join(claims[::-1])} and {claims[-1]}.")

	async def cmd_status(self, author, parts):
		await self.channel.send(('Hummus mode on, ' if self.hummus else '') +
			f"Zen mode on, time: {self.get_time_formatted()}, {self.strikes} strikes, "
			f"{self.get_solved_count()} out of {len(self.modules)} modules solved.")

	def get_random_unclaimed(self):
		return random.choice([module for module in self.modules if not module.solved and module.claim is None])

	async def cmd_claimany(self, author, parts):
		await self.get_random_unclaimed().handle_command("claim", author, parts)

	async def cmd_claimanyview(self, author, parts):
		await self.get_random_unclaimed().handle_command("claimview", author, parts)

	async def cmd_detonate(self, author, parts):
		if parts:
			await self.channel.send(f"{author.mention} Trailing arguments.")
			return
		if author.id == BOT_OWNER or isinstance(self.channel, discord.channel.DMChannel):
			await self.bomb_end(True)
			return
		else:
			msg = await self.channel.send(f"{author.mention} wants to detonate this bomb in an explosion-proof container instead of defusing it and selling the parts for :dollar:. If you agree, react with {DETONATE_REACT}")
			await msg.add_reaction(DETONATE_REACT)
			start_time = time.monotonic()
			while time.monotonic() < start_time + DETONATE_TIMEOUT:
				try:
					await self.client.wait_for('reaction_add', timeout=start_time+DETONATE_TIMEOUT-time.monotonic(),
						check=lambda reaction, user: reaction.emoji == DETONATE_REACT and reaction.message.id == msg.id)
				except asyncio.TimeoutError:
					pass
				msg = await self.channel.get_message(msg.id)
				approval = 0
				for reaction in msg.reactions:
					if reaction.emoji == DETONATE_REACT:
						async for user in reaction.users():
							if user.id != author.id:
								approval += 1
				if approval >= DETONATE_APPROVAL:
					await self.bomb_end(True)
					return
			await self.channel.send(f"Only {approval} out of {DETONATE_APPROVAL} needed people agreed. Not detonating.")

	async def cmd_find(self, author, parts):
		if not parts:
			await self.channel.send(f"{author.mention} What should I look for?")
			return

		needle = ' '.join(parts).lower()
		found = []
		for module in self.modules:
			if needle in module.display_name.lower():
				found.append(module)
		if not found:
			await self.channel.send(f"{author.mention} Sorry, I couldn't find anything.")
			return
		else:
			found = [f"{module} - {module.get_status()}" for module in found]
			if len(found) == 1:
				await self.channel.send(f"{author.mention} I could only find {found[1]}")
			else:
				await self.channel.send(f"{author.mention} Here's what I could find:\n" + '\n'.join(found))

	COMMANDS = {
		"edgework": cmd_edgework,
		"status": cmd_status,
		"unclaimed": cmd_unclaimed,
		"find": cmd_find,
		"claims": cmd_claims,
		"claimany": cmd_claimany,
		"claimanyview": cmd_claimanyview,
		"cvany": cmd_claimanyview,
		"detonate": cmd_detonate,
	}
