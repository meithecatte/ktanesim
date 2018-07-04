import random
import modules

class WhosOnFirst(modules.Module):
	display_name = "Who's on First"
	manual_name = "Who\u2019s on First"
	supports_hummus = True
	help_text = "`{cmd} push you're` or `{cmd} press press` to push a button. The phrase must match exactly."
	module_score = 4

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

	DISPLAY_WORDS_HUMMUS = {
		"YES": 4, "FIRST": 0, "DISPLAY": 5, "OKAY": 2, "SAYS": 4, "NOTHING": 0,
		"": 4, "BLANK": 3, "NO": 5, "LED": 5, "LEAD": 4, "READ": 3,
		"RED": 1, "REED": 5, "LEED": 0, "HOLD ON": 0, "YOU": 2, "YOU ARE": 1,
		"YOUR": 2, "YOU'RE": 3, "UR": 5, "THERE": 0, "THEY'RE": 5, "THEIR": 4,
		"THEY ARE": 4, "SEE": 4, "C": 4, "CEE": 1
	}

	PRECEDENCE = {
		"READY":   ["YES", "OKAY", "WHAT", "MIDDLE", "LEFT", "PRESS", "RIGHT", "BLANK", "READY", "NO", "FIRST", "UHHH", "NOTHING", "WAIT"],
		"FIRST":   ["LEFT", "OKAY", "YES", "MIDDLE", "NO", "RIGHT", "NOTHING", "UHHH", "WAIT", "READY", "BLANK", "WHAT", "PRESS", "FIRST"],
		"NO":      ["BLANK", "UHHH", "WAIT", "FIRST", "WHAT", "READY", "RIGHT", "YES", "NOTHING", "LEFT", "PRESS", "OKAY", "NO", "MIDDLE"],
		"BLANK":   ["WAIT", "RIGHT", "OKAY", "MIDDLE", "BLANK", "PRESS", "READY", "NOTHING", "NO", "WHAT", "LEFT", "UHHH", "YES", "FIRST"],
		"NOTHING": ["UHHH", "RIGHT", "OKAY", "MIDDLE", "YES", "BLANK", "NO", "PRESS", "LEFT", "WHAT", "WAIT", "FIRST", "NOTHING", "READY"],
		"YES":     ["OKAY", "RIGHT", "UHHH", "MIDDLE", "FIRST", "WHAT", "PRESS", "READY", "NOTHING", "YES", "LEFT", "BLANK", "NO", "WAIT"],
		"WHAT":    ["UHHH", "WHAT", "LEFT", "NOTHING", "READY", "BLANK", "MIDDLE", "NO", "OKAY", "FIRST", "WAIT", "YES", "PRESS", "RIGHT"],
		"UHHH":    ["READY", "NOTHING", "LEFT", "WHAT", "OKAY", "YES", "RIGHT", "NO", "PRESS", "BLANK", "UHHH", "MIDDLE", "WAIT", "FIRST"],
		"LEFT":    ["RIGHT", "LEFT", "FIRST", "NO", "MIDDLE", "YES", "BLANK", "WHAT", "UHHH", "WAIT", "PRESS", "READY", "OKAY", "NOTHING"],
		"RIGHT":   ["YES", "NOTHING", "READY", "PRESS", "NO", "WAIT", "WHAT", "RIGHT", "MIDDLE", "LEFT", "UHHH", "BLANK", "OKAY", "FIRST"],
		"MIDDLE":  ["BLANK", "READY", "OKAY", "WHAT", "NOTHING", "PRESS", "NO", "WAIT", "LEFT", "MIDDLE", "RIGHT", "FIRST", "UHHH", "YES"],
		"OKAY":    ["MIDDLE", "NO", "FIRST", "YES", "UHHH", "NOTHING", "WAIT", "OKAY", "LEFT", "READY", "BLANK", "PRESS", "WHAT", "RIGHT"],
		"WAIT":    ["UHHH", "NO", "BLANK", "OKAY", "YES", "LEFT", "FIRST", "PRESS", "WHAT", "WAIT", "NOTHING", "READY", "RIGHT", "MIDDLE"],
		"PRESS":   ["RIGHT", "MIDDLE", "YES", "READY", "PRESS", "OKAY", "NOTHING", "UHHH", "BLANK", "LEFT", "FIRST", "WHAT", "NO", "WAIT"],
		"YOU":     ["SURE", "YOU ARE", "YOUR", "YOU'RE", "NEXT", "UH HUH", "UR", "HOLD", "WHAT?", "YOU", "UH UH", "LIKE", "DONE", "U"],
		"YOU ARE": ["YOUR", "NEXT", "LIKE", "UH HUH", "WHAT?", "DONE", "UH UH", "HOLD", "YOU", "U", "YOU'RE", "SURE", "UR", "YOU ARE"],
		"YOUR":    ["UH UH", "YOU ARE", "UH HUH", "YOUR", "NEXT", "UR", "SURE", "U", "YOU'RE", "YOU", "WHAT?", "HOLD", "LIKE", "DONE"],
		"YOU'RE":  ["YOU", "YOU'RE", "UR", "NEXT", "UH UH", "YOU ARE", "U", "YOUR", "WHAT?", "UH HUH", "SURE", "DONE", "LIKE", "HOLD"],
		"UR":      ["DONE", "U", "UR", "UH HUH", "WHAT?", "SURE", "YOUR", "HOLD", "YOU'RE", "LIKE", "NEXT", "UH UH", "YOU ARE", "YOU"],
		"U":       ["UH HUH", "SURE", "NEXT", "WHAT?", "YOU'RE", "UR", "UH UH", "DONE", "U", "YOU", "LIKE", "HOLD", "YOU ARE", "YOUR"],
		"UH HUH":  ["UH HUH", "YOUR", "YOU ARE", "YOU", "DONE", "HOLD", "UH UH", "NEXT", "SURE", "LIKE", "YOU'RE", "UR", "U", "WHAT?"],
		"UH UH":   ["UR", "U", "YOU ARE", "YOU'RE", "NEXT", "UH UH", "DONE", "YOU", "UH HUH", "LIKE", "YOUR", "SURE", "HOLD", "WHAT?"],
		"WHAT?":   ["YOU", "HOLD", "YOU'RE", "YOUR", "U", "DONE", "UH UH", "LIKE", "YOU ARE", "UH HUH", "UR", "NEXT", "WHAT?", "SURE"],
		"DONE":    ["SURE", "UH HUH", "NEXT", "WHAT?", "YOUR", "UR", "YOU'RE", "HOLD", "LIKE", "YOU", "U", "YOU ARE", "UH UH", "DONE"],
		"NEXT":    ["WHAT?", "UH HUH", "UH UH", "YOUR", "HOLD", "SURE", "NEXT", "LIKE", "DONE", "YOU ARE", "UR", "YOU'RE", "U", "YOU"],
		"HOLD":    ["YOU ARE", "U", "DONE", "UH UH", "YOU", "UR", "SURE", "WHAT?", "YOU'RE", "NEXT", "HOLD", "UH HUH", "YOUR", "LIKE"],
		"SURE":    ["YOU ARE", "DONE", "LIKE", "YOU'RE", "YOU", "HOLD", "UH HUH", "UR", "SURE", "U", "WHAT?", "NEXT", "YOUR", "UH UH"],
		"LIKE":    ["YOU'RE", "NEXT", "U", "UR", "HOLD", "DONE", "UH UH", "WHAT?", "UH HUH", "YOU", "LIKE", "SURE", "YOU ARE", "YOUR"],
	}

	PRECEDENCE_HUMMUS = {
		"READY":   ["FIRST", "YES", "OKAY", "UHHH", "LEFT", "BLANK", "NOTHING", "RIGHT", "WHAT", "MIDDLE", "PRESS", "READY", "WAIT", "NO"],
		"FIRST":   ["NOTHING", "RIGHT", "UHHH", "NO", "BLANK", "WHAT", "READY", "MIDDLE", "WAIT", "PRESS", "LEFT", "OKAY", "FIRST", "YES"],
		"NO":      ["PRESS", "WAIT", "MIDDLE", "YES", "NO", "NOTHING", "WHAT", "BLANK", "RIGHT", "READY", "FIRST", "OKAY", "UHHH", "LEFT"],
		"BLANK":   ["PRESS", "OKAY", "READY", "BLANK", "WAIT", "UHHH", "WHAT", "LEFT", "RIGHT", "NO", "MIDDLE", "YES", "FIRST", "NOTHING"],
		"NOTHING": ["WAIT", "OKAY", "YES", "READY", "WHAT", "LEFT", "RIGHT", "BLANK", "PRESS", "NOTHING", "FIRST", "NO", "UHHH", "MIDDLE"],
		"YES":     ["WAIT", "YES", "WHAT", "UHHH", "READY", "OKAY", "MIDDLE", "PRESS", "RIGHT", "LEFT", "FIRST", "NO", "NOTHING", "BLANK"],
		"WHAT":    ["LEFT", "BLANK", "WAIT", "PRESS", "RIGHT", "MIDDLE", "FIRST", "OKAY", "NO", "READY", "YES", "UHHH", "WHAT", "NOTHING"],
		"UHHH":    ["LEFT", "YES", "READY", "MIDDLE", "RIGHT", "WHAT", "FIRST", "PRESS", "NO", "OKAY", "WAIT", "UHHH", "BLANK", "NOTHING"],
		"LEFT":    ["LEFT", "PRESS", "WAIT", "UHHH", "NOTHING", "MIDDLE", "NO", "FIRST", "OKAY", "WHAT", "YES", "READY", "RIGHT", "BLANK"],
		"RIGHT":   ["BLANK", "NO", "LEFT", "NOTHING", "FIRST", "YES", "RIGHT", "PRESS", "OKAY", "UHHH", "MIDDLE", "WAIT", "READY", "WHAT"],
		"MIDDLE":  ["UHHH", "NOTHING", "FIRST", "LEFT", "WHAT", "YES", "READY", "RIGHT", "NO", "MIDDLE", "WAIT", "BLANK", "PRESS", "OKAY"],
		"OKAY":    ["RIGHT", "OKAY", "FIRST", "WAIT", "LEFT", "READY", "PRESS", "MIDDLE", "WHAT", "NOTHING", "BLANK", "YES", "UHHH", "NO"],
		"WAIT":    ["FIRST", "WHAT", "OKAY", "LEFT", "BLANK", "WAIT", "UHHH", "NOTHING", "READY", "NO", "MIDDLE", "YES", "PRESS", "RIGHT"],
		"PRESS":   ["RIGHT", "NOTHING", "PRESS", "BLANK", "LEFT", "FIRST", "OKAY", "MIDDLE", "YES", "WHAT", "WAIT", "NO", "UHHH", "READY"],
		"YOU":     ["LIKE", "YOU", "UH HUH", "NEXT", "YOU'RE", "WHAT?", "UR", "UH UH", "U", "SURE", "DONE", "YOU ARE", "YOUR", "HOLD"],
		"YOU ARE": ["YOU ARE", "YOU'RE", "UR", "YOUR", "DONE", "SURE", "UH UH", "WHAT?", "YOU", "HOLD", "U", "LIKE", "NEXT", "UH HUH"],
		"YOUR":    ["UH HUH", "YOU ARE", "U", "NEXT", "YOU'RE", "YOUR", "UR", "SURE", "HOLD", "UH UH", "YOU", "LIKE", "WHAT?", "DONE"],
		"YOU'RE":  ["UR", "YOUR", "WHAT?", "YOU", "UH UH", "HOLD", "SURE", "UH HUH", "YOU ARE", "U", "LIKE", "DONE", "YOU'RE", "NEXT"],
		"UR":      ["DONE", "U", "YOU", "UH HUH", "WHAT?", "YOUR", "UH UH", "SURE", "UR", "LIKE", "HOLD", "NEXT", "YOU'RE", "YOU ARE"],
		"U":       ["WHAT?", "YOU'RE", "UR", "YOU ARE", "NEXT", "UH UH", "UH HUH", "U", "YOU", "DONE", "YOUR", "HOLD", "LIKE", "SURE"],
		"UH HUH":  ["UH HUH", "WHAT?", "UH UH", "DONE", "YOU'RE", "HOLD", "U", "UR", "NEXT", "YOU ARE", "LIKE", "YOU", "YOUR", "SURE"],
		"UH UH":   ["YOU", "YOU ARE", "LIKE", "YOUR", "UR", "U", "DONE", "SURE", "NEXT", "UH HUH", "UH UH", "HOLD", "WHAT?", "YOU'RE"],
		"WHAT?":   ["U", "DONE", "SURE", "YOUR", "NEXT", "YOU ARE", "YOU'RE", "HOLD", "LIKE", "UH HUH", "WHAT?", "UH UH", "UR", "YOU"],
		"DONE":    ["UR", "NEXT", "U", "YOU ARE", "YOU'RE", "SURE", "UH HUH", "YOUR", "LIKE", "DONE", "YOU", "WHAT?", "UH UH", "HOLD"],
		"NEXT":    ["LIKE", "NEXT", "UH HUH", "UR", "YOU ARE", "YOU'RE", "UH UH", "SURE", "U", "HOLD", "DONE", "YOUR", "YOU", "WHAT?"],
		"HOLD":    ["UH UH", "YOU ARE", "YOU", "SURE", "UR", "DONE", "UH HUH", "HOLD", "LIKE", "YOU'RE", "YOUR", "NEXT", "U", "WHAT?"],
		"SURE":    ["YOU", "UH HUH", "WHAT?", "UH UH", "U", "DONE", "YOUR", "SURE", "NEXT", "HOLD", "LIKE", "YOU'RE", "YOU ARE", "UR"],
		"LIKE":    ["YOUR", "U", "UH UH", "UR", "WHAT?", "YOU ARE", "YOU", "NEXT", "UH HUH", "YOU'RE", "DONE", "LIKE", "SURE", "HOLD"],
	}

	def __init__(self, bomb, ident):
		super().__init__(bomb, ident)
		self.stage = 0
		self.randomize()
	
	def get_svg(self, led):
		svg = (
			f'<svg viewBox="0 0 348 348" fill="#fff" stroke-linecap="butt" stroke-linejoin="round" stroke-miterlimit="10">'
			f'<path stroke="#000" stroke-width="2" d="M5 5h338v338h-338z"/>'
			f'<circle fill="{led}" stroke="#000" cx="298" cy="40.5" r="15" stroke-width="2"/>'
			'<path fill="#000" stroke="#000" stroke-width="2" d="M34 25h230v67h-232zM277 106h52v208h-52z"/>'
			'<path stroke="#000" d="M34 125h106v44h-106zM158 125h106v44h-106zM34 202h106v44h-106zM158 202h106v44h-106zM34 270h106v44h-106zM158 270h106v44h-106z"/>' +
			'<path fill="{color}" stroke="{color}" stroke-width="2" d="M290 208h28v16h-28z"/>'.format(color='#0f0' if self.stage > 2 else '#fff') +
			'<path fill="{color}" stroke="{color}" stroke-width="2" d="M290 245h28v16h-28z"/>'.format(color='#0f0' if self.stage > 1 else '#fff') +
			'<path fill="{color}" stroke="{color}" stroke-width="2" d="M290 281h28v16h-28z"/>'.format(color='#0f0' if self.stage > 0 else '#fff') +
			'<text x="150" y="72" text-anchor="middle" style="font-family:sans-serif;font-size:28pt;">{:s}</text>'.format(self.display))
		for index, text in enumerate(self.buttons):
			x = [87, 211][index % 2]
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
		button = button.replace("‘", "'");
		button = button.replace("`", "'");
		button = button.replace("’", "'");
		button = button.replace("′", "'");

		if button not in sum(WhosOnFirst.BUTTON_GROUPS, []):
			return await self.bomb.channel.send(f"{author.mention} \"{button}\" isn't a valid word.")

		if button not in self.buttons:
			return await self.handle_unsubmittable(author)

		solution = self.get_solution()

		self.log(f"Pressed {button}, expected {solution}")

		if button == solution:
			self.stage += 1
			if self.stage == 3:
				await self.handle_solve(author)
			else:
				self.randomize()
				await self.handle_next_stage(author)
		else:
			self.randomize()
			await self.handle_strike(author)
	
	def get_solution(self):
		if self.bomb.hummus:
			index = WhosOnFirst.DISPLAY_WORDS_HUMMUS[self.display]
		else:
			index = WhosOnFirst.DISPLAY_WORDS[self.display]

		word = self.buttons[index]
		self.log(f"Button to look at is {index}, the word is {word}")

		if self.bomb.hummus:
			precedence = WhosOnFirst.PRECEDENCE_HUMMUS[word]
		else:
			precedence = WhosOnFirst.PRECEDENCE[word]

		self.log(f"Precedence list: {','.join(precedence)}")
		for button in precedence:
			if button in self.buttons:
				self.log(f"Button to press is {button}")
				return button
		assert False, "The rules should prevent this"

	COMMANDS = {
		"push": cmd_push,
		"press": cmd_push,
	}
