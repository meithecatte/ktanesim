import random
import modules
from config import *

class WhosOnFirst(modules.Module):
	display_name = "Who's on First"
	manual_name = "Who's on First"
	supports_hummus = False
	help_text = "`{cmd} push you're` to push a button. The phrase must match exactly."
	module_score = 4
	strike_penalty = 6

	BUTTON_GROUPS = [
		["READY", "FIRST", "NO", "BLANK", "NOTHING", "YES", "WHAT", "UHHH", "LEFT", "RIGHT", "MIDDLE", "OKAY", "WAIT", "PRESS"],
		["YOU", "YOU ARE", "YOUR", "YOU'RE", "UR", "U", "UH HUH", "UH UH", "WHAT?", "DONE", "NEXT", "HOLD", "SURE", "LIKE"],
	]

	DISPLAY_WORDS = {
		"YES": 2, "FIRST": 1, "DISPLAY": 5, "OKAY": 1, "SAYS": 5, "NOTHING": 2,
		"": 4, "BLANK": 3, "NO": 5, "LED": 2, "LEAD": 5, "READ": 3,
		"RED": 3, "REED": 4, "LEED": 4, "HOLD ON": 5, "YOU": 3, "YOU ARE": 5,
		"YOUR": 3, "YOU'RE": 3, "UR": 0, "THERE": 5, "THEY'RE": 4, "THEIR": 3,
		"THEY ARE": 2, "SEE": 5, "C": 1, "CEE": 5
	}

	PRECEDENCE = {
		"READY":  ["YES", "OKAY", "WHAT", "MIDDLE", "LEFT", "PRESS", "RIGHT", "BLANK", "READY", "NO", "FIRST", "UHHH", "NOTHING", "WAIT"],
		"FIRST":  ["LEFT", "OKAY", "YES", "MIDDLE", "NO", "RIGHT", "NOTHING", "UHHH", "WAIT", "READY", "BLANK", "WHAT", "PRESS", "FIRST"],
		"NO":  ["BLANK", "UHHH", "WAIT", "FIRST", "WHAT", "READY", "RIGHT", "YES", "NOTHING", "LEFT", "PRESS", "OKAY", "NO", "MIDDLE"],
		"BLANK":  ["WAIT", "RIGHT", "OKAY", "MIDDLE", "BLANK", "PRESS", "READY", "NOTHING", "NO", "WHAT", "LEFT", "UHHH", "YES", "FIRST"],
		"NOTHING":  ["UHHH", "RIGHT", "OKAY", "MIDDLE", "YES", "BLANK", "NO", "PRESS", "LEFT", "WHAT", "WAIT", "FIRST", "NOTHING", "READY"],
		"YES":  ["OKAY", "RIGHT", "UHHH", "MIDDLE", "FIRST", "WHAT", "PRESS", "READY", "NOTHING", "YES", "LEFT", "BLANK", "NO", "WAIT"],
		"WHAT":  ["UHHH", "WHAT", "LEFT", "NOTHING", "READY", "BLANK", "MIDDLE", "NO", "OKAY", "FIRST", "WAIT", "YES", "PRESS", "RIGHT"],
		"UHHH":  ["READY", "NOTHING", "LEFT", "WHAT", "OKAY", "YES", "RIGHT", "NO", "PRESS", "BLANK", "UHHH", "MIDDLE", "WAIT", "FIRST"],
		"LEFT":  ["RIGHT", "LEFT", "FIRST", "NO", "MIDDLE", "YES", "BLANK", "WHAT", "UHHH", "WAIT", "PRESS", "READY", "OKAY", "NOTHING"],
		"RIGHT":  ["YES", "NOTHING", "READY", "PRESS", "NO", "WAIT", "WHAT", "RIGHT", "MIDDLE", "LEFT", "UHHH", "BLANK", "OKAY", "FIRST"],
		"MIDDLE":  ["BLANK", "READY", "OKAY", "WHAT", "NOTHING", "PRESS", "NO", "WAIT", "LEFT", "MIDDLE", "RIGHT", "FIRST", "UHHH", "YES"],
		"OKAY":  ["MIDDLE", "NO", "FIRST", "YES", "UHHH", "NOTHING", "WAIT", "OKAY", "LEFT", "READY", "BLANK", "PRESS", "WHAT", "RIGHT"],
		"WAIT":  ["UHHH", "NO", "BLANK", "OKAY", "YES", "LEFT", "FIRST", "PRESS", "WHAT", "WAIT", "NOTHING", "READY", "RIGHT", "MIDDLE"],
		"PRESS":  ["RIGHT", "MIDDLE", "YES", "READY", "PRESS", "OKAY", "NOTHING", "UHHH", "BLANK", "LEFT", "FIRST", "WHAT", "NO", "WAIT"],
		"YOU":  ["SURE", "YOU ARE", "YOUR", "YOU'RE", "NEXT", "UH HUH", "UR", "HOLD", "WHAT?", "YOU", "UH UH", "LIKE", "DONE", "U"],
		"YOU ARE":  ["YOUR", "NEXT", "LIKE", "UH HUH", "WHAT?", "DONE", "UH UH", "HOLD", "YOU", "U", "YOU'RE", "SURE", "UR", "YOU ARE"],
		"YOUR":  ["UH UH", "YOU ARE", "UH HUH", "YOUR", "NEXT", "UR", "SURE", "U", "YOU'RE", "YOU", "WHAT?", "HOLD", "LIKE", "DONE"],
		"YOU'RE":  ["YOU", "YOU'RE", "UR", "NEXT", "UH UH", "YOU ARE", "U", "YOUR", "WHAT?", "UH HUH", "SURE", "DONE", "LIKE", "HOLD"],
		"UR":  ["DONE", "U", "UR", "UH HUH", "WHAT?", "SURE", "YOUR", "HOLD", "YOU'RE", "LIKE", "NEXT", "UH UH", "YOU ARE", "YOU"],
		"U":  ["UH HUH", "SURE", "NEXT", "WHAT?", "YOU'RE", "UR", "UH UH", "DONE", "U", "YOU", "LIKE", "HOLD", "YOU ARE", "YOUR"],
		"UH HUH":  ["UH HUH", "YOUR", "YOU ARE", "YOU", "DONE", "HOLD", "UH UH", "NEXT", "SURE", "LIKE", "YOU'RE", "UR", "U", "WHAT?"],
		"UH UH":  ["UR", "U", "YOU ARE", "YOU'RE", "NEXT", "UH UH", "DONE", "YOU", "UH HUH", "LIKE", "YOUR", "SURE", "HOLD", "WHAT?"],
		"WHAT?":  ["YOU", "HOLD", "YOU'RE", "YOUR", "U", "DONE", "UH UH", "LIKE", "YOU ARE", "UH HUH", "UR", "NEXT", "WHAT?", "SURE"],
		"DONE":  ["SURE", "UH HUH", "NEXT", "WHAT?", "YOUR", "UR", "YOU'RE", "HOLD", "LIKE", "YOU", "U", "YOU ARE", "UH UH", "DONE"],
		"NEXT":  ["WHAT?", "UH HUH", "UH UH", "YOUR", "HOLD", "SURE", "NEXT", "LIKE", "DONE", "YOU ARE", "UR", "YOU'RE", "U", "YOU"],
		"HOLD":  ["YOU ARE", "U", "DONE", "UH UH", "YOU", "UR", "SURE", "WHAT?", "YOU'RE", "NEXT", "HOLD", "UH HUH", "YOUR", "LIKE"],
		"SURE":  ["YOU ARE", "DONE", "LIKE", "YOU'RE", "YOU", "HOLD", "UH HUH", "UR", "SURE", "U", "WHAT?", "NEXT", "YOUR", "UH UH"],
		"LIKE":  ["YOU'RE", "NEXT", "U", "UR", "HOLD", "DONE", "UH UH", "WHAT?", "UH HUH", "YOU", "LIKE", "SURE", "YOU ARE", "YOUR"],
	}

	def __init__(self, bomb, ident):
		super().__init__(bomb, ident)
		self.stage = 0
		self.randomize()
	
	def get_svg(self, led):
		svg = (
			'<svg viewBox="0.0 0.0 348.0 348.0" fill="#fff" stroke-linecap="butt" stroke-linejoin="round" stroke-miterlimit="10" xmlns="http://www.w3.org/2000/svg">'
			'<path stroke="#000" stroke-width="2" d="M5.079 5.776h336.913v337.67H5.08z"/>'
			'<path fill="#000" stroke="#000" stroke-width="2" d="M34.772 25.39h232.41v67.559H34.771zM277.462 106.383h52.661v209.04h-52.661z"/>'
			'<path stroke="#000" d="M34.772 125.226h106.645v44.82H34.772zM157.407 125.226h106.645v44.82H157.407zM34.772 202.318h106.645v44.819H34.772zM157.407 202.318h106.645v44.819H157.407zM34.772 270.604h106.645v44.819H34.772zM157.407 270.604h106.645v44.819H157.407z"/>' +
			f'<path fill="{led}" stroke="#000" stroke-width="2" d="M282.734 40.554c0-8.376 6.966-15.165 15.56-15.165 4.126 0 8.084 1.597 11.001 4.441 2.918 2.844 4.558 6.702 4.558 10.724 0 8.376-6.966 15.165-15.56 15.165-8.593 0-15.559-6.79-15.559-15.165z"/>' +
			'<path fill="{color}" stroke="{color}" stroke-width="2" d="M290.06 208.903h27.465v16.472H290.06z"/>'.format(color='#0f0' if self.stage > 2 else '#fff') +
			'<path fill="{color}" stroke="{color}" stroke-width="2" d="M290.06 245.113h27.465v16.472H290.06z"/>'.format(color='#0f0' if self.stage > 1 else '#fff') +
			'<path fill="{color}" stroke="{color}" stroke-width="2" d="M290.06 281.323h27.465v16.472H290.06z"/>'.format(color='#0f0' if self.stage > 0 else '#fff') +
			'<text x="151" y="72" text-anchor="middle" style="font-family:sans-serif;font-size:28pt;">{:s}</text>'.format(self.display))
		for index, text in enumerate(self.buttons):
			x = [88, 211][index % 2]
			y = [156, 233, 301][index // 2]
			svg += f'<text x="{x}" y="{y}" text-anchor="middle" style="font-family:sans-serif;font-size:16pt;" fill="#000">{text}</text>'
		svg += '</svg>'
		return svg
	
	def randomize(self):
		self.display = random.choice(list(WhosOnFirst.DISPLAY_WORDS.keys()))
		self.buttons = random.sample(random.choice(WhosOnFirst.BUTTON_GROUPS), 6)
		self.log(f"State randomized. Stage {self.stage}. Display: {self.display}. Buttons: {' '.join(self.buttons)}")
	
	@modules.check_solve_cmd
	async def cmd_push(self, author, parts):
		if not parts:
			return await self.usage(author)
		button = ' '.join(parts).upper()

		if button not in self.buttons:
			return await self.do_view(f"{author.mention} There is no `{button}` button on the module!")

		solution = self.get_solution()

		self.log(f"Pressed {button}, expected {solution}")

		if button == solution:
			self.stage += 1
			if self.stage == 3:
				await self.handle_solved(author)
			else:
				self.randomize()
				await self.do_view(f"{author.mention} Good! Next stage:")
		else:
			self.randomize()
			await self.handle_strike(author)
	
	def get_solution(self):
		index = WhosOnFirst.DISPLAY_WORDS[self.display]
		word = self.buttons[index]
		self.log(f"Button to look at is {index}, the word is {word}")
		precedence = WhosOnFirst.PRECEDENCE[word]
		self.log(f"Precedence list: {','.join(precedence)}")
		for button in precedence:
			if button in self.buttons:
				self.log(f"Button to press is {button}")
				return button
		assert False, "The rules should prevent this"

	COMMANDS = {
		"push": cmd_push
	}
