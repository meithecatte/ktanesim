import random
import asyncio
import modules
from config import *

class Button(modules.Module):
	display_name = "The Button"
	manual_name = "The Button"
	supports_hummus = True
	help_text = "`{cmd} tap` to tap, `{cmd} hold` to hold, `{cmd} release 7` to release when any digit of the timer is 7."
	module_score = 1
	strike_penalty = 6

	LABELS = ["ABORT", "DETONATE", "HOLD", "PRESS"]
	COLORS = {
		"red": "#ff0000",
		"blue": "#0000ff",
		"yellow": "#ffff00",
		"white": "#ffffff"
	}

	def __init__(self, bomb, ident):
		super().__init__(bomb, ident)
		self.button_label = random.choice(Button.LABELS)
		self.button_color = random.choice(list(Button.COLORS.keys()))
		self.log("button label: {:s}".format(self.button_label))
		self.log("button color: {:s}".format(self.button_color))
		self.strip_color = None

	def get_svg(self):
		solvelight = 'fill="#00ff00"' if self.solved else ''
		svg = '<svg viewBox="0.0 0.0 348.0 348.0" fill="none" stroke="none" strike-linejoin="round" stroke-linecap="butt" stroke-miterlimit="10">'
		svg += '<path stroke="#000" fill="#fff" stroke-width="2" d="M5.079 5.776h336.913v337.67H5.08z"/>'
		svg += '<path stroke="#000" fill="#ccc" stroke-width="2" d="M55.186 58.892H81.14v12.284H55.186zM180.336 58.892h25.953v12.284h-25.953zM84.142 63.423h92.252v7.748H84.142z"/>'
		svg += '<path stroke="#000" stroke-width="2" d="M273.456 109.96h46.268v196.567h-46.268z" fill="{:s}"/>'\
			.format(Button.COLORS[self.strip_color] if self.strip_color is not None else '#000')
		svg += f'<path stroke="#000" {solvelight} stroke-width="2" d="M282.734 40.554c0-8.376 6.966-15.165 15.56-15.165 4.126 0 8.084 1.597 11.001 4.441 2.918 2.844 4.558 6.702 4.558 10.724 0 8.376-6.966 15.165-15.56 15.165-8.593 0-15.559-6.79-15.559-15.165z"/>'
		svg += '<path stroke="#000" stroke-width="2" d="M17.037 71.178h225.386v235.339H17.037z"{:s}/>'\
			.format(' fill="#000" fill-opacity="0.3"' if self.strip_color is None else '')
		svg += '<path fill="{:s}" stroke="#000" stroke-width="2" d="M31.556 188.848c0-54.95 43.954-99.496 98.174-99.496 26.037 0 51.008 10.482 69.419 29.141 18.41 18.66 28.754 43.967 28.754 70.355 0 54.95-43.954 99.496-98.173 99.496-54.22 0-98.174-44.546-98.174-99.496z"/>'\
			.format(Button.COLORS[self.button_color])
		svg += '<text x="130" y="200" fill="{:s}" style="font-size:24pt;font-family:sans-serif;" text-anchor="middle">{:s}</text>'\
			.format('#fff' if self.button_color in ['red', 'blue'] else '#000', self.button_label)
		if self.strip_color is None:
			svg += '<path stroke="#000" d="M40.97 95.997H218.89v185.701H40.97zM17.709 72.895L41.173 95.95M219.097 281.696l23.087 24.283M242.157 72.895L219.512 95.95M40.764 281.7l-22.646 23.056"/>'
		svg += '</svg>'
		return svg

	@modules.check_solve_cmd
	@modules.noparts
	async def cmd_tap(self, author):
		if self.should_hold():
			self.log("tapping - should hold")
			await self.handle_strike(author)
		else:
			self.log("tapping - correct")
			await self.handle_solved(author)
	
	@modules.check_solve_cmd
	@modules.noparts
	async def cmd_hold(self, author):
		self.strip_color = random.choice(list(Button.COLORS.keys()))
		self.log('start holding, strip color: {:s}'.format(self.strip_color))
		await self.do_view(f"{author.mention} The button is being held.")
	
	@modules.check_solve_cmd
	async def cmd_release(self, author, parts):
		if not parts or not parts[0].isdigit() or len(parts[0]) != 1:
			await self.usage(author)
		elif self.strip_color is None:
			await self.bomb.channel.send(f"{author.mention} The button is not being held. Hold it with `!{PREFIX}{self.ident} hold` first.")
		else:
			answer = parts[0]
			time = self.bomb.get_time_formatted()
			while answer not in time:
				await asyncio.sleep(0.5)
				time = self.bomb.get_time_formatted()
			expected = self.get_release_digit()
			self.log("Releasing at {:s}, expected {:d}, player answered {:s}".format(time, expected, answer))
			self.strip_color = None
			should_hold = self.should_hold()
			self.log("should{:s} hold".format("n't" if not should_hold else ''))
			if should_hold and str(expected) in time:
				await self.handle_solved(author)
			else:
				await self.handle_strike(author)
		
	def should_hold(self):
		if self.bomb.hummus:
			if self.button_color == "yellow" and self.button_label == "PRESS":
				self.log('rule: yellow PRESS')
				return True
			elif self.button_color == "white" and self.bomb.has_lit_indicator("BOB"):
				self.log('rule: white with lit BOB')
				return False
			elif self.bomb.get_battery_count() > 1:
				self.log('rule: more than one battery')
				return True
			elif self.button_color == "red":
				self.log('rule: red')
				return False
			elif self.button_color == "blue":
				self.log('rule: blue')
				return True
			elif self.bomb.get_battery_count() > 2:
				# never reached because of the one battery rule
				self.log('rule: more than two batteries')
				return False
			else:
				self.log('rule: wildcard')
				return True
		else:
			if self.button_color == "blue" and self.button_label == "ABORT":
				self.log('rule: blue ABORT')
				return True
			elif self.bomb.get_battery_count() > 1 and self.button_label == "DETONATE":
				self.log('rule: more than one battery and DETONATE')
				return False
			elif self.button_color == "white" and self.bomb.has_lit_indicator("CAR"):
				self.log('rule: white with lit CAR')
				return True
			elif self.bomb.get_battery_count() > 2 and self.bomb.has_lit_indicator("FRK"):
				self.log('rule: more than two batteries and lit FRK')
				return False
			elif self.button_color == "yellow":
				self.log('rule: yellow')
				return True
			elif self.button_color == "red" and self.button_label == "HOLD":
				self.log('rule: red HOLD')
				return False
			else:
				self.log('rule: wildcard')
				return True

	def get_release_digit(self):
		if self.bomb.hummus:
			return {"white": 3, "yellow": 3, "red": 5, "blue": 4}[self.strip_color]
		else:
			return {"blue": 4, "white": 1, "yellow": 5, "red": 1}[self.strip_color]
	
	COMMANDS = {
		"tap": cmd_tap,
		"hold": cmd_hold,
		"release": cmd_release
	}
