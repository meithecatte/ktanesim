import random
import enum

class BatteryWidget:
	def __init__(self, bomb):
		self.battery_count = random.randint(1, 2)

@enum.unique
class Indicator(enum.Enum):
	SND = enum.auto()
	CLR = enum.auto()
	CAR = enum.auto()
	IND = enum.auto()
	FRQ = enum.auto()
	SIG = enum.auto()
	NSA = enum.auto()
	MSA = enum.auto()
	TRN = enum.auto()
	BOB = enum.auto()
	FRK = enum.auto()

class IndicatorWidget:
	def __init__(self, bomb):
		possible_indicators = list(set(Indicator) - set(ind.code for ind in bomb.edgework if isinstance(ind, IndicatorWidget)))
		assert possible_indicators, "Somehow all 11 indicators were used even though 5 is the limit"
		self.code = random.choice(possible_indicators)
		self.lit = random.random() > 0.4

	def __str__(self):
		return ('*' if self.lit else '') + self.code.name

@enum.unique
class PortType(enum.Enum):
	Serial = enum.auto()
	Parallel = enum.auto()
	DVI = enum.auto()
	PS2 = enum.auto()
	RJ45 = enum.auto()
	StereoRCA = enum.auto()

class PortPlateWidget:
	PORT_GROUPS = [
		[PortType.Serial, PortType.Parallel],
		[PortType.DVI, PortType.PS2,
		 PortType.RJ45, PortType.StereoRCA]
	]

	def __init__(self, bomb):
		group = random.choice(self.PORT_GROUPS)
		self.ports = []
		for port in group:
			if random.random() > 0.5:
				self.ports.append(port)

	def __str__(self):
		return '[' + (', '.join(port.name for port in self.ports) if self.ports else 'Empty') + ']'

WIDGETS = [BatteryWidget, IndicatorWidget, PortPlateWidget]
