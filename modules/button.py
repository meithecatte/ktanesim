import random
import asyncio
import modules

class Button(modules.Module):
	display_name = "The Button"
	manual_name = "The Button"
	supports_hummus = True
	help_text = "`{cmd} tap` to tap, `{cmd} hold` to hold, `{cmd} release 7` to release when any digit of the timer is 7."
	module_score = 1

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
		self.release_pending = None

	def get_svg(self, led):
		svg = (
			f'<svg viewBox="0 0 348 348" fill="none" stroke="none" strike-linejoin="round" stroke-linecap="butt" stroke-miterlimit="1">'
			f'<path stroke="#000" fill="#fff" stroke-width="2" d="M5 5h338v338h-338z"/>'
			f'<path stroke="#000" fill="#000" fill-opacity="0.1" stroke-width="2" d="M54 59h26v12h-26zm127 0h26v12h-26zm-97 4h92v8h-92z"/>'
			f'<path stroke="#000" stroke-width="2" d="M273 110h45v196h-45z" fill="{Button.COLORS[self.strip_color] if self.strip_color is not None else "#000"}"/>'
			f'<circle fill="{led}" stroke="#000" cx="298" cy="40.5" r="15" stroke-width="2"/>')
		if self.strip_color is None:
			svg += '<path fill="#000" fill-opacity="0.1" stroke="#000" stroke-width="2" d="M17 71h225v235H17z"/>'
		else:
			svg += ('<path stroke-width="1.5" stroke="#000" fill="#000" fill-opacity="0.1" d="M17 63l24-36h177l24 36z"/>'
				'<path stroke-width="1.5" stroke="#000" fill="#000" fill-opacity="0.1" d="M17 63l16-8l20-20l-12-8zm36-28l-12-8h177l-12 8zm153 0l12-8l24 36l-16-8z"/>'
				'<path stroke-width="2" stroke="#000" fill="#000" fill-opacity="0.1" d="M33 55l20-20h153l20 20z"/>')

		svg += f'<circle fill="{Button.COLORS[self.button_color]}" stroke="#000" stroke-width="2" r="100" cx="130" cy="189"/>'
		text_color = '#fff' if self.button_color in ['red', 'blue'] else '#000'
		svg += f'<text x="130" y="200" fill="{text_color}" style="font-size:24pt;font-family:sans-serif;" text-anchor="middle">{self.button_label}</text>'
		if self.strip_color is None:
			svg += '<path stroke-width="1.5" stroke="#000" d="M17 71l24 24h177l24-24M17 306l24-24h177l24 24M41 95v187M218 95v187"/>'
		svg += '</svg>'
		return svg

	@modules.check_solve_cmd
	@modules.noparts
	async def cmd_tap(self, author):
		if self.strip_color is not None:
			return await self.bomb.channel.send(f"{author.mention} I'm already holding the button. How am I supposed to tap it too? I could release it, but you need to tell me when.")

		if self.should_hold():
			self.log("tapping - should hold")
			await self.handle_strike(author)
		else:
			self.log("tapping - correct")
			await self.handle_solve(author)
	
	@modules.check_solve_cmd
	@modules.noparts
	async def cmd_hold(self, author):
		if self.strip_color is not None:
			return await self.do_view(f"{author.mention} The button is already being held.")

		self.strip_color = random.choice(list(Button.COLORS.keys()))
		self.log('start holding, strip color: {:s}'.format(self.strip_color))
		await self.do_view(f"{author.mention} The button is being held.")
	
	@modules.check_solve_cmd
	async def cmd_release(self, author, parts):
		if not parts or not parts[0].isdigit() or len(parts[0]) != 1:
			await self.usage(author)
		elif self.strip_color is None:
			await self.bomb.channel.send(f"{author.mention} The button is not being held. Hold it with `!{PREFIX}{self.ident} hold` first.")
		elif self.release_pending is not None:
			await self.bomb.channel.send(f"{author.mention} I'm already releasing the button when I see a {self.release_pending}.")
		else:
			answer = parts[0]
			self.release_pending = answer
			time = self.bomb.get_time_formatted()
			while answer not in time:
				await asyncio.sleep(0.5)
				time = self.bomb.get_time_formatted()
			expected = self.get_release_digit()
			self.log("Releasing at {:s}, expected {:d}, player answered {:s}".format(time, expected, answer))
			self.strip_color = None
			self.release_pending = None
			should_hold = self.should_hold()
			self.log("should{:s} hold".format("n't" if not should_hold else ''))
			if should_hold and str(expected) in time:
				await self.handle_solve(author)
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
