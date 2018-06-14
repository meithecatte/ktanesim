import random
import modules

class Keypad(modules.Module):
	display_name = "Keypad"
	manual_name = "Keypad"
	supports_hummus = True
	help_text = "`{cmd} press 1 3 2 4` or `{cmd} press 1324` or `{cmd} press tl bl tr br`. Partial solutions allowed."
	module_score = 1
	strike_penalty = 6
	BUTTONS = ['tl', 'tr', 'bl', 'br']

	def __init__(self, bomb, ident):
		super().__init__(bomb, ident)

		if self.bomb.hummus:
			self.column = random.choice([
				["six", "ce", "k", "hollowstar", "paragraph", "mirrorc", "zeta"],
				["ce", "n", "lambda", "at", "chair", "paragraph", "k"],
				["kitty", "six", "h", "zeta", "pitchfork", "lambda", "eye"],
				["kitty", "smiley", "circle", "copyright", "lightning", "mirrorc", "h"],
				["q", "copyright", "questionmark", "alien", "hollowstar", "fullstar", "chair"],
				["ae", "r", "euro", "alien", "bt", "eye", "pitchfork"]])
		else:
			self.column = random.choice([
				["q", "at", "lambda", "lightning", "kitty", "h", "mirrorc"],
				["euro", "q", "mirrorc", "ce", "hollowstar", "h", "questionmark"],
				["copyright", "eye", "ce", "k", "r", "lambda", "hollowstar"],
				["six", "paragraph", "bt", "kitty", "k", "questionmark", "smiley"],
				["pitchfork", "smiley", "bt", "normalc", "paragraph", "three", "fullstar"],
				["six", "euro", "tracks", "ae", "pitchfork", "n", "omega"]])

		self.log(f"Precedence list: {' '.join(self.column)}")
		self.buttons = random.sample(self.column, 4)
		self.log(f"Buttons: {' '.join(self.buttons)}")
		self.led = ['#000'] * 4
		self.progress = 0
		self.solution = []

		for button in self.column:
			if button in self.buttons:
				self.solution.append(self.buttons.index(button))
		self.log(f"Solution: {' '.join(map(str, self.solution))}")

	def get_svg(self):
		solvelight = 'fill="#00ff00"' if self.solved else ''
		return (f'<svg viewBox="0.0 0.0 348.0 348.0" fill="#fff" stroke-linecap="butt" stroke-linejoin="round" stroke-miterlimit="10" xmlns:xlink="http://www.w3.org/1999/xlink">'
		       f'<path stroke="#000" stroke-width="2" d="M5.079 5.776h336.913v337.67H5.08z"/>'
		       f'<path stroke="#000" d="M38.37 95.737h107.213v100.189H38.37zM152.48 95.737h107.213v100.189H152.48zM38.37 206.52h107.213v100.19H38.37zM152.48 206.52h107.213v100.19H152.48z"/>'
		       f'<path stroke="#000" {solvelight} stroke-width="2" d="M282.734 40.554c0-8.376 6.966-15.165 15.56-15.165 4.126 0 8.084 1.597 11.001 4.441 2.918 2.844 4.558 6.702 4.558 10.724 0 8.376-6.966 15.165-15.56 15.165-8.593 0-15.559-6.79-15.559-15.165z"/>'
		       f'<image xlink:href="img/keypad/{self.buttons[0]}.png" width="90" height="90" x="47" y="105"/>'
		       f'<image xlink:href="img/keypad/{self.buttons[1]}.png" width="90" height="90" x="161" y="105"/>'
		       f'<image xlink:href="img/keypad/{self.buttons[2]}.png" width="90" height="90" x="47" y="215"/>'
		       f'<image xlink:href="img/keypad/{self.buttons[3]}.png" width="90" height="90" x="161" y="215"/>'
		       f'<path stroke="{self.led[0]}" fill="{self.led[0]}" stroke-width="2" d="M82.102 102.05h19.748v5.638H82.102z"/>'
		       f'<path stroke="{self.led[1]}" fill="{self.led[1]}" stroke-width="2" d="M196.213 102.05h19.748v5.638h-19.748z"/>'
		       f'<path stroke="{self.led[2]}" fill="{self.led[2]}" stroke-width="2" d="M82.102 213.525h19.748v5.638H82.102z"/>'
		       f'<path stroke="{self.led[3]}" fill="{self.led[3]}" stroke-width="2" d="M196.213 213.525h19.748v5.638h-19.748z"/>'
		       f'</svg>')
	
	@modules.check_solve_cmd
	async def cmd_press(self, author, parts):
		if not parts:
			await self.usage(author)
			return

		input_ = []
		for part in parts:
			part = part.lower()
			if part.isdigit():
				self.log(f"Digit input: {part}")
				input_ += [int(digit) - 1 for digit in part]
			elif part in Keypad.BUTTONS:
				self.log(f"Named input: {part}")
				input_.append(Keypad.BUTTONS.index(part))
			else:
				await self.usage(author)
				return

		self.log(f"Parsed input: {' '.join(map(str, input_))}")
		while input_ and not self.solved:
			press = input_.pop(0)
			expected = self.solution[self.progress]
			self.log(f"Stage {self.progress}, expected {expected}, got {press}")
			if expected == press:
				self.log("Correct button pressed")
				self.led[press] = '#0f0'
				self.progress += 1
				if self.progress == 4:
					await self.handle_solved(author)
					return
			else:
				if self.led[press] == '#0f0':
					self.log(f"Button {press} has already been pressed, ignoring")
				else:
					self.led[press] = '#f00'
					await self.handle_strike(author)
					self.led[press] = '#000'
					return
		await self.do_view(f"{author.mention}")
	
	COMMANDS = {**modules.Module.COMMANDS,
		"press": cmd_press
	}
