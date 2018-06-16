import random
import cairosvg
import modules
from functools import lru_cache
from wand.image import Image

MORSE_CODE = {
	"a": ".-",
	"b": "-...",
	"c": "-.-.",
	"d": "-..",
	"e": ".",
	"f": "..-.",
	"g": "--.",
	"h": "....",
	"i": "..",
	"j": ".---",
	"k": "-.-",
	"l": ".-..",
	"m": "--",
	"n": "-.",
	"o": "---",
	"p": ".--.",
	"q": "--.-",
	"r": ".-.",
	"s": "...",
	"t": "-",
	"u": "..-",
	"v": "...-",
	"w": ".--",
	"x": "-..-",
	"y": "-.--",
	"z": "--..",
}

DOT_LENGTH = 45

class MorseCode(modules.Module):
	display_name = "Morse Code"
	manual_name = "Morse Code"
	supports_hummus = True
	help_text = "`{cmd} tx 3.545`, `{cmd} tx 545`, `{cmd} tx 3.545 MHz`, or `{cmd} transmit ...` to transmit on 3.545 MHz."
	module_score = 3

	WORDS = {
		"shell":  505,
		"halls":  515,
		"slick":  522,
		"trick":  532,
		"boxes":  535,
		"leaks":  542,
		"strobe": 545,
		"bistro": 552,
		"flick":  555,
		"bombs":  565,
		"break":  572,
		"brick":  575,
		"steak":  582,
		"sting":  592,
		"vector": 595,
		"beats":  600,
	}

	WORDS_HUMMUS = {
		"bravo":  502,
		"alien":  505,
		"slick":  512,
		"strobe": 515,
		"break":  532,
		"beats":  535,
		"bombs":  545,
		"halls":  552,
		"shell":  562,
		"vector": 565,
		"brick":  572,
		"hello":  575,
		"flick":  582,
		"brain":  585,
		"sting":  595,
		"trick":  600,
	}

	def __init__(self, bomb, ident):
		super().__init__(bomb, ident)

		self.wordset = MorseCode.WORDS_HUMMUS if self.bomb.hummus else MorseCode.WORDS
		self.word = random.choice(list(self.wordset.keys()))
		self.frequency = self.wordset[self.word]
		self.last_frequency = 502 if self.bomb.hummus else 505
		self.log(f"The word is {self.word}, with a frequency of 3.{self.frequency} MHz")
	
	def get_image(self, rx_led, solve_led):
		svg = (
			f'<svg viewBox="0 0 348 348" fill="#fff" stroke-linecap="butt" stroke-linejoin="round" stroke-miterlimit="10">'
			f'<path stroke="#000" stroke-width="2" d="M5.079 5.776h336.913v337.67H5.08zM43.346 139.811h252.756v30.33H43.346zM59.22 170.948v-9.764M84.055 139.811v12.819M133.725 139.408v12.819M108.89 170.545v-9.764M233.065 139.407v12.819M208.23 170.543v-9.763M282.735 139.407v12.819M257.9 170.543v-9.763M158.56 170.545v-9.764M183.395 139.408v12.819M154.46 50.575l119.054.094M36.633 50.575L6.113 6.7M117.832 290.612h87.906v34.803h-87.906zM16.483 118.441h302.835v157.543H16.483z"/>'
			f'<path fill="{solve_led}" stroke="#000" stroke-width="2" d="M282.734 40.554c0-8.376 6.966-15.165 15.56-15.165 4.126 0 8.084 1.597 11.001 4.441 2.918 2.844 4.558 6.702 4.558 10.724 0 8.376-6.966 15.165-15.56 15.165-8.593 0-15.559-6.79-15.559-15.165z"/>'
			f'<path fill="#000" d="M58.661 187.43H277.34v72.032H58.66z"/>'
			f'<path fill="{"#ff0" if rx_led else "#fff"}" stroke="#000" stroke-width="2" d="M36.633 50.575c0-8.376 26.376-15.166 58.913-15.166 32.537 0 58.913 6.79 58.913 15.166 0 8.375-26.376 15.165-58.913 15.165-32.537 0-58.913-6.79-58.913-15.165z"/>'
			f'<path fill="#000" stroke="#000" stroke-width="2" d="M45.79 23.2h12.882v53.354H45.79zM132.357 23.2h12.882v53.354h-12.882zM48.307 196.769l-22.866 26.677 22.866 26.677zM287.693 196.769l22.866 26.677-22.866 26.677zM{(self.last_frequency - 500) * 236 / 100 + 47} 136.147h8.535v40.283h-8.535z"/>'
			f'<text x="161.785" y="318" text-anchor="middle" fill="#000" style="font-size:20pt;font-family:sans-serif;">TX</text>'
			f'<text x="168" y="237" text-anchor="middle" style="font-size:28pt;font-family:sans-serif;">3.{self.last_frequency} MHz</text>'
			f'</svg>')
		return cairosvg.svg2png(svg.encode())

	def render(self, strike):
		if self.solved:
			return self.get_image(False, '#0f0'), 'render.png'
		
		led = '#f00' if strike else '#fff'

		on = self.get_image(True, led)
		off = self.get_image(False, led)

		with Image() as im:
			def add(frame, units):
				modules.gif_append(im, frame, units * DOT_LENGTH)

			for letter in self.word:
				add(off, 3)
				first_signal = True
				for signal in MORSE_CODE[letter]:
					if not first_signal: add(off, 1)
					first_signal = False
					add(on, 3 if signal == '-' else 1)
			add(off, 4)

			return modules.gif_output(im)
	
	@modules.check_solve_cmd
	async def cmd_transmit(self, author, parts):
		if len(parts) == 0 or len(parts) > 2 or len(parts) == 2 and parts[1].lower() != "mhz":
			return await self.usage(author)

		freq = parts[0]
		if freq.startswith('3.'): freq = freq[2:]
	
		if not freq.isdigit() or len(freq) != 3 or (freq[0] != "5" or freq[-1] not in '25') and freq != "600":
			return await self.usage(author)

		freq = int(freq)
		freqs = sorted(list(self.wordset.values()))
		if freq not in freqs:
			if freq < freqs[0]:
				self.last_frequency = freqs[0]
			elif freq > freqs[-1]:
				self.last_frequency = freqs[-1]
			elif freq < self.last_frequency:
				# moving left
				self.last_frequency = max(f for f in freqs if f < freq)
			else:
				# moving right
				self.last_frequency = min(f for f in freqs if f > freq)
			return await self.handle_unsubmittable(author)

		self.last_frequency = freq

		if freq == self.frequency:
			await self.handle_solve(author)
		else:
			await self.handle_strike(author)
	
	COMMANDS = {
		"transmit": cmd_transmit,
		"tx": cmd_transmit,
	}
