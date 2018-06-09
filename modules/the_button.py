import random
import io
import cairosvg
from bomb import Module

class TheButton(Module):
	display_name = "The Button"
	manual_name = "The Button"
	supports_hummus = True
	help_text = "`{prefix}{ident} tap` to tap, `{prefix}{ident} hold` to hold, `{prefix}{ident} release 7` to release when any digit of the timer is 7."
	module_score = 1
	strike_penalty = 6

	LABELS = ["ABORT", "DETONATE", "HOLD", "PRESS"]
	COLORS = {
		"red": "#ff0000",
		"blue": "#0000ff",
		"yellow": "#ffff00",
		"white": "#ececec"
	}

	def __init__(self, bomb, ident):
		super().__init__(bomb, ident)
		self.button_label = random.choice(TheButton.LABELS)
		self.button_color = random.choice(list(TheButton.COLORS.keys()))
		self.strip_color = None

	def render(self):
		svg = '<svg viewBox="0.0 0.0 348.0 348.0" fill="none" stroke="none" stroke-linecap="square" stroke-miterlimit="10">'
		svg += '<path stroke="#000" fill="#fff" stroke-width="2" stroke-linejoin="round" stroke-linecap="butt" d="M5.079 5.776h336.913v337.67H5.08z"/>'
		svg += '<path stroke="#000" fill="#ccc" stroke-width="2" stroke-linejoin="round" stroke-linecap="butt" d="M55.186 58.892H81.14v12.284H55.186zM180.336 58.892h25.953v12.284h-25.953zM84.142 63.423h92.252v7.748H84.142z"/>'
		svg += '<path stroke="#000" stroke-width="2" stroke-linejoin="round" stroke-linecap="butt" d="M273.456 109.96h46.268v196.567h-46.268z" fill="{:s}"/>'\
			.format(TheButton.COLORS[self.strip_color] if self.strip_color is not None else '#000')
		svg += '<path stroke="#000" {:s} stroke-width="2" stroke-linejoin="round" stroke-linecap="butt" d="M282.734 40.554c0-8.376 6.966-15.165 15.56-15.165 4.126 0 8.084 1.597 11.001 4.441 2.918 2.844 4.558 6.702 4.558 10.724 0 8.376-6.966 15.165-15.56 15.165-8.593 0-15.559-6.79-15.559-15.165z"/>'\
			.format('fill="#0f0" ' if self.solved else '')
		svg += '<path stroke="#000" stroke-width="2" stroke-linejoin="round" stroke-linecap="butt" d="M17.037 71.178h225.386v235.339H17.037z"{:s}/>'\
			.format(' fill="#000" fill-opacity="0.3"' if self.strip_color is None else '')
		svg += '<path fill="{:s}" stroke="#000" stroke-width="2" stroke-linejoin="round" stroke-linecap="butt" d="M31.556 188.848c0-54.95 43.954-99.496 98.174-99.496 26.037 0 51.008 10.482 69.419 29.141 18.41 18.66 28.754 43.967 28.754 70.355 0 54.95-43.954 99.496-98.173 99.496-54.22 0-98.174-44.546-98.174-99.496z"/>'\
			.format(TheButton.COLORS[self.button_color])
		svg += '<text x="130" y="200" fill="{:s}" style="font-size:24pt;font-family:sans-serif;" text-anchor="middle">{:s}</text>'\
			.format('#fff' if self.button_color in ['red', 'blue'] else '#000', self.button_label)
		if self.strip_color is None:
			svg += '<path stroke="#000" stroke-linejoin="round" stroke-linecap="butt" d="M40.97 95.997H218.89v185.701H40.97zM17.709 72.895L41.173 95.95M219.097 281.696l23.087 24.283M242.157 72.895L219.512 95.95M40.764 281.7l-22.646 23.056"/>'
		svg += '</svg>'
		return io.BytesIO(cairosvg.svg2png(svg.encode('utf-8'))), 'render.png'
