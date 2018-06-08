import time
import random

class Module:
	def __init__(self, bomb):
		self.bomb = bomb
		self.solved = False
		self.claim = None
	
	def get_manual(self):
		if self.supports_hummus and bomb.hummus:
			getparam = '?VanillaRuleSeed = 2'
		else:
			getparam = ''

		return 'https://ktane.timwi.de/manual/{:s}.html{:s}'.format(self.manual_name, getparam)

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
		for module in modules:
			self.modules.append(module(self))
		self.hummus = hummus
		self.strikes = 0
		self.start_time = time.monotonic()

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

	def _randomize_serial(self):
		def get_any():
			return random.choice(Bomb.SERIAL_NUMBER_CHARACTERS)
		def get_letter():
			return random.choice(Bomb.SERIAL_NUMBER_CHARACTERS[:-10])
		def get_digit():
			return str(random.randint(0, 9))

		return get_any() + get_any() + get_digit() + get_letter() + get_letter() + get_digit()
