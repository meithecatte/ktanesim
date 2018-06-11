import time
import random
from urllib.parse import quote as urlencode
import discord
import io
import cairosvg
import leaderboard
import bomb_manager
from config import *

class Module:
	def __init__(self, bomb, ident):
		self.bomb = bomb
		self.ident = ident
		self.solved = False
		self.claim = None
		self.take_pending = None
		self.log_data = []

	def __str__(self):
		return '{:s} (#{:d})'.format(self.display_name, self.ident)

	def log(self, msg):
		self.log_data.append((self.bomb.get_time_formatted(), msg))
	
	def get_log(self):
		return ['[{:s}@{:s}] {:s}'.format(x[0], str(self), x[1]) for x in self.log_data]

	def get_manual(self):
		if self.supports_hummus and self.bomb.hummus:
			getparam = '?VanillaRuleSeed=2'
		else:
			getparam = ''

		return 'https://ktane.timwi.de/manual/{:s}.html{:s}'.format(urlencode(self.manual_name), getparam)

	def get_help(self):
		return self.help_text.format(prefix=PREFIX, ident=self.ident)

	async def usage(self, msg):
		await msg.channel.send("{:s} {:s}".format(msg.author.mention, self.get_help()))

	async def handle_solved(self, msg):
		self.log('module solved')
		self.solved = True
		leaderboard.record_solve(msg.author, self.module_score)
		await self.cmd_view(msg, "{:s} solved {:s}. {:d} {:s} been awarded.".format(msg.author.mention, str(self), self.module_score, 'points have' if self.module_score > 1 else 'point has'))
		if self.bomb.get_solved_count() == len(self.bomb.modules):
			await bomb_manager.defused(msg.channel)

	async def handle_strike(self, msg):
		self.log('strike!')
		self.bomb.strikes += 1
		leaderboard.record_strike(msg.author, self.strike_penalty)
		await self.cmd_view(msg, "{:s} got a strike. -{:d} point{:s} from {:s}".format(str(self), self.strike_penalty, 's' if self.strike_penalty > 1 else '', msg.author.mention))

	def render(self):
		return io.BytesIO(cairosvg.svg2png(self.get_svg().encode('utf-8'), unsafe=True)), 'render.png'

	async def cmd_take(self, msg):
		if self.claim is None:
			await msg.channel.send("{:s} {:s} is not claimed by anybody. Type `{prefix}{ident} claim` to claim it.".format(msg.author.mention, prefix=PREFIX, ident=self.ident))
		elif self.claim.id == msg.author.id:
			await msg.channel.send("{:s} You already claimed this module. Did you mean `{prefix}{ident} mine`?".format(msg.author.mention, prefix=PREFIX, ident=self.ident))
		elif self.take_pending is not None:
			await msg.channel.send("{:s} {:s} has already issued a `take` command.".format(msg.author.mention, str(self.take_pending)))
		else:
			self.take_pending = msg.author
			await msg.channel.send("{:s} {:s} wants to take {:s}. Type `{prefix}{ident} mine` within {:d} seconds to confirm you are still working on the module"
				.format(self.claim.mention, str(msg.author), str(self), TAKE_TIMEOUT, prefix=PREFIX, timeout=TIMEOUT))
	async def cmd_view(self, msg, text):
		stream, filename = self.render()
		embed = discord.Embed(title=str(self), description='[Manual]({:s}). {:s}'.format(self.get_manual(), self.get_help())).set_image(url="attachment://"+filename)
		await msg.channel.send(text, file=discord.File(stream, filename=filename), embed=embed)

	async def cmd_claim(self, msg):
		if self.solved:
			await msg.channel.send("{:s} {:s} has been solved already.".format(msg.author.mention, str(self)))
		elif self.claim is not None:
			if self.claim.id == msg.author.id:
				await msg.channel.send("{:s} You have already claimed {:s}.".format(msg.author.mention, str(self)))
			else:
				await msg.channel.send("{:s} Sorry, {:s} has already been claimed by {:s}. If you think they have abandoned it, you may type `{prefix}{:d} take` to free it up."
					.format(msg.author.mention, str(self), str(self.claim), self.ident, prefix=PREFIX))
		elif len(self.bomb.get_claims(msg.author)) >= MAX_CLAIMS_PER_PLAYER:
			await msg.channel.send("{:s} Sorry, you can only claim {:d} modules at once. Try `{prefix}claims`."
				.format(msg.author.mention, MAX_CLAIMS_PER_PLAYER, prefix=PREFIX))
		else:
			self.claim = msg.author
			return True
		return False

	async def cmd_unclaim(self, msg):
		if self.claim and self.claim.id == msg.author.id:
			self.claim = None
			await msg.channel.send("{:s} has unclaimed {:s}".format(msg.author.mention, str(self)))
		else:
			await msg.channel.send("{:s} You did not claim {:s}, so you can't unclaim it.".format(msg.author.mention, str(self)))

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

	def __init__(self, modules, hummus = False):
		self.start_time = time.monotonic()
		self.hummus = hummus
		self.strikes = 0
		self.serial = self._randomize_serial()
		self.edgework = []
		for _ in range(5):
			self.edgework.append(random.choice(Bomb.EDGEWORK_WIDGETS)())
		self.modules = []
		for index, module in enumerate(modules):
			self.modules.append(module(self, index + 1))

	def get_log(self):
		log = ["Edgework: {:s}".format(self.get_edgework())]
		for module in self.modules:
			log += module.get_log()
		return '\n'.join(log)

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
