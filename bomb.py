import time
import random
import discord
from config import *

class Module:
	def __init__(self, bomb, ident):
		self.bomb = bomb
		self.ident = ident
		self.solved = False
		self.claim = None
	
	def get_manual(self):
		if self.supports_hummus and self.bomb.hummus:
			getparam = '?VanillaRuleSeed=2'
		else:
			getparam = ''

		return 'https://ktane.timwi.de/manual/{:s}.html{:s}'.format(self.manual_name, getparam)
	
	def get_help(self):
		return self.help_text.format(prefix=PREFIX, ident=self.ident)
	def __str__(self):
		return '{:s} (#{:d})'.format(self.display_name, self.ident)
	
	def handle_solved(self):
		self.solved = True
		self.claim = None
	
	def render(self):
		return open('placeholder.jpg', 'rb'), 'render.jpg'
	
	async def cmd_view(self, msg, text):
		stream, filename = self.render()
		embed = discord.Embed(title=str(self), description='[Manual]({:s}). {:s}'.format(self.get_manual(), self.get_help())).set_image(url="attachment://"+filename)
		await msg.channel.send(text, file=discord.File(stream, filename=filename), embed=embed)

	async def cmd_claim(self, msg):
		if self.solved:
			await msg.channel.send("{:s} {:s} has been solved already.".format(msg.author.mention, str(self)))
		if self.claim is not None:
			if self.claim.id == msg.author.id:
				await msg.channel.send("{:s} You have already claimed {:s}.".format(msg.author.mention, str(self)))
			else:
				await msg.channel.send("{:s} Sorry, {:s} has already been claimed by {:s}. Did you mean `{prefix}{:d} take`?"
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
			await msg.channel.send("{:s} has unclaimed {:s}".format(msg.author.id, str(self)))
		else:
			await msg.channel.send("{:s} You did not claim {:s}, so you can't unclaim it.".format(msg.author.id, str(self)))

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
		self.serial = self._randomize_serial()
		self.edgework = []
		for _ in range(5):
			self.edgework.append(random.choice(Bomb.EDGEWORK_WIDGETS)())
		self.modules = []
		for index, module in enumerate(modules):
			self.modules.append(module(self, index + 1))
		self.hummus = hummus
		self.strikes = 0
		self.start_time = time.monotonic()

	def get_claims(self, user):
		return [module for module in self.modules if module.claim is not None and module.claim.id == user.id]

	def get_widgets(self, type_):
		return list(filter(lambda widget: type(widget) is type_, self.edgework))

	def get_battery_count(self):
		return sum(widget.battery_count for widget in self.get_widgets(BatteryWidget))
	
	def get_holder_count(self):
		return len(self.get_widgets(BatteryWidget))
	
	def get_edgework(self):
		edgework = [
			'{:d}B {:d}H'.format(self.get_battery_count(), self.get_holder_count()),
			' '.join(map(str, self.get_widgets(IndicatorWidget))),
			' '.join(map(str, self.get_widgets(PortPlateWidget))),
			self.serial]
		return ' // '.join(widget for widget in edgework if widget != '')
	
	def get_unclaimed(self):
		return [module for module in self.modules if module.claim is None]

	def get_time(self):
		return time.monotonic() - self.start_time
	
	def get_time_formatted(self):
		seconds = self.get_time()
		minutes = int(seconds // 60)
		seconds %= 60
		hours = minutes // 60
		minutes %= 60
		return '{:d}:{:02d}:{:05.2f}'.format(hours, minutes, seconds)
	
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
