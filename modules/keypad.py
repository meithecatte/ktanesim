import random
from bomb import Module

class Keypad(Module):
	display_name = "Keypad"
	manual_name = "Keypad"
	supports_hummus = True
	help_text = "`{prefix}{ident} press 1 3 2 4` or `{prefix}{ident} press 1324` or `{prefix}{ident} press tl bl tr br`. Partial solutions allowed."
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

		self.log("Precedence list: {:s}".format(' '.join(self.column)))
		self.buttons = random.sample(self.column, 4)
		self.log("Buttons: {:s}".format(' '.join(self.buttons)))
		self.led = ['#000'] * 4
		self.progress = 0
		self.solution = []

		for button in self.column:
			if button in self.buttons:
				self.solution.append(self.buttons.index(button))
		self.log("Solution: {:s}".format(' '.join(map(str, self.solution))))

	def get_svg(self):
		svg = '<svg viewBox="0.0 0.0 348.0 348.0" fill="#fff" stroke-linecap="butt" stroke-linejoin="round" stroke-miterlimit="10" xmlns:xlink="http://www.w3.org/1999/xlink">'
		svg += '<path stroke="#000" stroke-width="2" d="M5.079 5.776h336.913v337.67H5.08z"/>'
		svg += '<path stroke="#000" d="M38.37 95.737h107.213v100.189H38.37zM152.48 95.737h107.213v100.189H152.48zM38.37 206.52h107.213v100.19H38.37zM152.48 206.52h107.213v100.19H152.48z"/>'
		svg += '<path stroke="#000" {:s} stroke-width="2" d="M282.734 40.554c0-8.376 6.966-15.165 15.56-15.165 4.126 0 8.084 1.597 11.001 4.441 2.918 2.844 4.558 6.702 4.558 10.724 0 8.376-6.966 15.165-15.56 15.165-8.593 0-15.559-6.79-15.559-15.165z"/>'\
			.format('fill="#0f0"' if self.solved else '')
		svg += '<image xlink:href="img/keypad/{:s}.png" width="90" height="90" x="47" y="105"/>'.format(self.buttons[0])
		svg += '<image xlink:href="img/keypad/{:s}.png" width="90" height="90" x="161" y="105"/>'.format(self.buttons[1])
		svg += '<image xlink:href="img/keypad/{:s}.png" width="90" height="90" x="47" y="215"/>'.format(self.buttons[2])
		svg += '<image xlink:href="img/keypad/{:s}.png" width="90" height="90" x="161" y="215"/>'.format(self.buttons[3])
		svg += '<path stroke="{fill}" fill="{fill}" stroke-width="2" d="M82.102 102.05h19.748v5.638H82.102z"/>'.format(fill=self.led[0])
		svg += '<path stroke="{fill}" fill="{fill}" stroke-width="2" d="M196.213 102.05h19.748v5.638h-19.748z"/>'.format(fill=self.led[1])
		svg += '<path stroke="{fill}" fill="{fill}" stroke-width="2" d="M82.102 213.525h19.748v5.638H82.102z"/>'.format(fill=self.led[2])
		svg += '<path stroke="{fill}" fill="{fill}" stroke-width="2" d="M196.213 213.525h19.748v5.638h-19.748z"/>'.format(fill=self.led[3])
		svg += '</svg>'
		return svg
	
	async def command(self, msg, parts):
		self.log("Command: {:s}".format(' '.join(parts)))
		if len(parts) < 2 or parts[0] != "press":
			await self.usage(msg)
			return

		input_ = []
		for part in parts[1:]:
			part = part.lower()
			if part.isdigit():
				self.log("Digit input: {:s}".format(part))
				input_ += [int(digit) - 1 for digit in part]
			elif part in Keypad.BUTTONS:
				self.log("Named input: {:s}".format(part))
				input_.append(Keypad.BUTTONS.index(part))
			else:
				await self.usage(msg)
				return

		self.log("Parsed input: {:s}".format(' '.join(map(str, input_))))
		while input_ and not self.solved:
			press = input_.pop(0)
			expected = self.solution[self.progress]
			self.log("Stage {:d}, expected {:d}, got {:d}".format(self.progress, expected, press))
			if expected == press:
				self.log("Correct button pressed")
				self.led[press] = '#0f0'
				self.progress += 1
				if self.progress == 4:
					print("Module solved")
					await self.handle_solved(msg)
					return
			else:
				if self.led[press] == '#0f0':
					self.log("Button {:d} has already been pressed, ignoring".format(press))
				else:
					self.led[press] = '#f00'
					await self.handle_strike(msg)
					self.led[press] = '#000'
					return
		await self.cmd_view(msg, '')
