import random
import cairosvg
import io
import modules
from wand.image import Image

class SimonSays(modules.Module):
	display_name = "Simon Says"
	manual_name = "Simon Says"
	supports_hummus = True
	help_text = "`{cmd} press red green blue yellow`, `{cmd} press rgby`. You must include the input from any previous stages."
	module_score = 3

	COLORS = ["red", "green", "blue", "yellow"]

	MAPPING = {
	# strikes, vowel, hummus
		(0, True, False):  {"red": "blue", "blue": "red", "green": "yellow", "yellow": "green"},
		(1, True, False):  {"red": "yellow", "blue": "green", "green": "blue", "yellow": "red"},
		(2, True, False):  {"red": "green", "blue": "red", "green": "yellow", "yellow": "blue"},
		(0, False, False): {"red": "blue", "blue": "yellow", "green": "green", "yellow": "red"},
		(1, False, False): {"red": "red", "blue": "blue", "green": "yellow", "yellow": "green"},
		(2, False, False): {"red": "yellow", "blue": "green", "green": "blue", "yellow": "red"},
		(0, True, True):   {"red": "yellow", "blue": "red", "green": "yellow", "yellow": "blue"},
		(1, True, True):   {"red": "green", "blue": "red", "green": "green", "yellow": "green"},
		(2, True, True):   {"red": "yellow", "blue": "yellow", "green": "yellow", "yellow": "green"},
		(0, False, True):  {"red": "red", "blue": "yellow", "green": "red", "yellow": "red"},
		(1, False, True):  {"red": "blue", "blue": "blue", "green": "blue", "yellow": "green"},
		(2, False, True):  {"red": "yellow", "blue": "red", "green": "yellow", "yellow": "yellow"}
	}

	for strikes in range(3):
		for vowel in True, False:
			for hummus in True, False:
				row = MAPPING[strikes, vowel, hummus]
				assert set(row.keys()) == set(COLORS)
				assert set(row.values()) - set(COLORS) == set()

	def __init__(self, bomb, ident):
		super().__init__(bomb, ident)
		self.progress = 0
		self.sequence = []
		for _ in range(random.randint(3, 5)):
			self.sequence.append(random.choice(["red", "green", "blue", "yellow"]))

		self.log(f"Sequence: {' '.join(self.sequence)}")
	
	def add_frame(self, im, color, delay, led):
		im.sequence.append(Image(blob=self.get_image(color, led), format='png'))
		with im.sequence[-1] as frame:
			frame.delay = delay
	
	def get_image(self, color, led):
		svg = ('<svg viewBox="0.0 0.0 348.0 348.0" fill="#fff" stroke-linecap="butt" stroke-linejoin="round" stroke-miterlimit="10">'
			'<path stroke="#000" stroke-width="2" d="M5.079 5.776h336.913v337.67H5.08z"/>' +
			f'<path fill="{led}" stroke="#000" stroke-width="2" d="M282.734 40.554c0-8.376 6.966-15.165 15.56-15.165 4.126 0 8.084 1.597 11.001 4.441 2.918 2.844 4.558 6.702 4.558 10.724 0 8.376-6.966 15.165-15.56 15.165-8.593 0-15.559-6.79-15.559-15.165z"/>' +
			'<path fill="{:s}" stroke="#000" stroke-width="2" d="M68.695 174.611l52.567-52.567 52.566 52.567-52.566 52.567z"/>'.format('#f00' if color == "red" else '#300') +
			'<path fill="{:s}" stroke="#000" stroke-width="2" d="M121.262 227.178l52.567-52.567 52.567 52.567-52.567 52.567z"/>'.format('#0f0' if color == "green" else '#030') +
			'<path fill="{:s}" stroke="#000" stroke-width="2" d="M121.262 122.044l52.567-52.567 52.567 52.567-52.567 52.567z"/>'.format('#00f' if color == "blue" else '#003') +
			'<path fill="{:s}" stroke="#000" stroke-width="2" d="M173.829 174.611l52.567-52.567 52.567 52.567-52.567 52.567z"/>'.format('#ff0' if color == "yellow" else '#330') +
			'</svg>')
		return cairosvg.svg2png(svg.encode('utf-8'))

	def render(self, strike):
		if self.solved:
			return self.get_image(None, '#0f0'), 'render.png'

		led = '#f00' if strike else '#fff'

		with Image() as im:
			self.add_frame(im, None, 200, led)

			for color in self.sequence[:self.progress+1]:
				self.add_frame(im, color, 60, led)
				self.add_frame(im, None, 10, led)

			self.add_frame(im, None, 140, led) # the delay in the original game is 5 seconds. I've reduced it to 3.5 seconds

			im.type = 'optimize'
			im.format = 'gif'
			return im.make_blob(), 'render.gif'
	
	@modules.check_solve_cmd
	async def cmd_press(self, author, parts):
		if not parts:
			await self.bomb.channel.send(f"{author.mention} What should I press?")
			return

		parsed = []
		for part in parts:
			part = part.lower()
			if part in SimonSays.COLORS:
				parsed.append(part)
			else:
				for letter in part:
					short_names = {x[:1]: x for x in SimonSays.COLORS}
					if letter in short_names:
						parsed.append(short_names[letter])
					else:
						await self.bomb.channel.send(f"{author.mention} Neither `{part}` nor `{letter}` is a color.")
						return
		self.log(f"Parsed: {' '.join(parsed)}")
		small_progress = 0
		solution = self.get_solution()
		self.log(f"Solution: {' '.join(solution)}")
		success = False # whether the input advanced the stage of the module
		for press in parsed:
			self.log(f"Pressing {press}")
			expected = solution[small_progress]
			self.log(f"Expected {expected}")
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
		self.log(f"Strikes: {strikes}. Vowel: {vowel}.")
		return [mapping[color] for color in self.sequence]
	
	COMMANDS = {
		"press": cmd_press
	}
