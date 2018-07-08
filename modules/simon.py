import io
import random
import imageio
import cairosvg
import enum
import modules
from functools import lru_cache

class SimonSays(modules.Module):
	display_name = "Simon Says"
	manual_name = "Simon Says"
	supports_hummus = True
	help_text = "`{cmd} press red green blue yellow`, `{cmd} press rgby`. You must include the input from any previous stages."
	module_score = 3

	class Color(enum.Enum):
		red = enum.auto()
		green = enum.auto()
		blue = enum.auto()
		yellow = enum.auto()

	MAPPING = {
	# strikes, vowel, hummus
		(0, True, False):  {Color.red: Color.blue, Color.blue: Color.red, Color.green: Color.yellow, Color.yellow: Color.green},
		(1, True, False):  {Color.red: Color.yellow, Color.blue: Color.green, Color.green: Color.blue, Color.yellow: Color.red},
		(2, True, False):  {Color.red: Color.green, Color.blue: Color.red, Color.green: Color.yellow, Color.yellow: Color.blue},
		(0, False, False): {Color.red: Color.blue, Color.blue: Color.yellow, Color.green: Color.green, Color.yellow: Color.red},
		(1, False, False): {Color.red: Color.red, Color.blue: Color.blue, Color.green: Color.yellow, Color.yellow: Color.green},
		(2, False, False): {Color.red: Color.yellow, Color.blue: Color.green, Color.green: Color.blue, Color.yellow: Color.red},
		(0, True, True):   {Color.red: Color.yellow, Color.blue: Color.red, Color.green: Color.yellow, Color.yellow: Color.blue},
		(1, True, True):   {Color.red: Color.green, Color.blue: Color.red, Color.green: Color.green, Color.yellow: Color.green},
		(2, True, True):   {Color.red: Color.yellow, Color.blue: Color.yellow, Color.green: Color.yellow, Color.yellow: Color.green},
		(0, False, True):  {Color.red: Color.red, Color.blue: Color.yellow, Color.green: Color.red, Color.yellow: Color.red},
		(1, False, True):  {Color.red: Color.blue, Color.blue: Color.blue, Color.green: Color.blue, Color.yellow: Color.green},
		(2, False, True):  {Color.red: Color.yellow, Color.blue: Color.red, Color.green: Color.yellow, Color.yellow: Color.yellow}
	}

	def __init__(self, bomb, ident):
		super().__init__(bomb, ident)
		self.progress = 0
		self.sequence = []
		for _ in range(random.randint(3, 5)):
			self.sequence.append(random.choice(list(SimonSays.Color)))

		self.log(f"Sequence: {' '.join(color.name for color in self.sequence)}")
	
	@staticmethod
	@lru_cache(maxsize=16)
	def get_image(color, led, as_array):
		svg = (
			f'<svg viewBox="4 4 340 340" fill="#fff" stroke-linecap="butt" stroke-linejoin="round" stroke-miterlimit="10">'
			f'<path stroke="#000" stroke-width="2" d="M5 5h338v338h-338z"/>'
			f'<circle fill="{led}" stroke="#000" cx="298" cy="40.5" r="15" stroke-width="2"/>'
			'<path fill="{:s}" stroke="#000" stroke-width="2" d="M68 174l52-52 52 52-52 52z"/>'.format('#f00' if color == SimonSays.Color.red else '#300') +
			'<path fill="{:s}" stroke="#000" stroke-width="2" d="M120 226l52-52 52 52-52 52z"/>'.format('#0f0' if color == SimonSays.Color.green else '#030') +
			'<path fill="{:s}" stroke="#000" stroke-width="2" d="M120 122l52-52 52 52-52 52z"/>'.format('#00f' if color == SimonSays.Color.blue else '#003') +
			'<path fill="{:s}" stroke="#000" stroke-width="2" d="M172 174l52-52 52 52-52 52z"/>'.format('#ff0' if color == SimonSays.Color.yellow else '#330') +
			'</svg>')
		data = cairosvg.svg2png(svg.encode())
		return imageio.imread(data, 'PNG-PIL') if as_array else data

	def render(self, strike):
		if self.solved:
			return io.BytesIO(SimonSays.get_image(None, '#0f0', False)), 'render.png'

		led = '#f00' if strike else '#fff'

		frames = []
		durations = []

		def add(color, delay):
			frames.append(SimonSays.get_image(color, led, True))
			durations.append(delay)

		add(None, 2)

		first = True
		for color in self.sequence[:self.progress+1]:
			if not first:
				add(None, 0.1)

			add(color, 0.6)
			first = False

		return modules.gif_output(frames, durations, 0)
	
	@modules.check_solve_cmd
	async def cmd_press(self, author, parts):
		if not parts:
			return await self.bomb.channel.send(f"{author.mention} What should I press?")

		parsed = []
		short_names = {x.name[:1]: x for x in SimonSays.Color}
		for part in parts:
			part = part.lower()
			try:
				parsed.append(SimonSays.Color[part])
			except KeyError:
				for letter in part:
					try:
						parsed.append(short_names[letter])
					except KeyError:
						return await self.bomb.channel.send(f"{author.mention} Neither `{part}` nor `{letter}` is a color.")

		self.log(f"Parsed: {' '.join(color.name for color in parsed)}")
		small_progress = 0
		solution = self.get_solution()
		success = False # whether the input advanced the stage of the module
		for press in parsed:
			expected = solution[small_progress]
			self.log(f"Pressing {press.name}, expected {expected.name}")
			if expected != press:
				await self.handle_strike(author)
				return
			else:
				if small_progress >= self.progress:
					self.progress += 1
					small_progress = 0
					success = True
					if self.progress >= len(self.sequence):
						await self.handle_solve(author)
						return
				else:
					small_progress += 1

		if success:
			await self.handle_next_stage(author)
		else:
			await self.do_view(f"{author.mention} All of these inputs are correct, but the module expects more")
	
	def get_solution(self):
		strikes = self.bomb.strikes
		if strikes > 2: strikes = 2
		vowel = self.bomb.has_vowel()
		mapping = SimonSays.MAPPING[strikes, vowel, self.bomb.hummus]
		solution = [mapping[color] for color in self.sequence]
		self.log(f"Strikes: {strikes}. Vowel: {vowel}. Solution: {' '.join(color.name for color in solution)}")
		return solution
	
	COMMANDS = {
		"press": cmd_press
	}
