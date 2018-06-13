import random
import modules

class Wires(modules.Module):
	display_name = "Wires"
	manual_name = "Wires"
	supports_hummus = True
	help_text = "`{cmd} cut 3` to cut the third wire. Empty spaces are not counted."
	module_score = 1
	strike_penalty = 6

	COLORS = {
		"black": "#000000",
		"blue":  "#0000ff",
		"red":   "#ff0000",
		"white": "#dddddd",
		"yellow":"#ffff00"
	}

	PATHS_UNCUT = [
		"m61.6063 94.05481c10.847099 -2.326851 19.528023 12.062485 30.566925 13.165352c24.493225 2.4470444 49.259254 -2.853485 73.83202 -1.4094467c11.687805 0.68683624 20.134766 12.74498 31.509186 15.519684c25.011337 6.1013184 50.6969 9.060364 76.183716 12.695541",
		"m65.367455 130.26443c24.469223 -11.123505 52.597984 13.950012 79.47506 13.637787c23.982635 -0.27859497 49.307022 -7.900284 71.95276 0c18.927505 6.603134 43.66768 30.634155 57.842514 16.45932",
		"m62.076115 163.18391c13.594257 -1.7824554 28.332623 -9.683853 40.913387 -4.233597c8.545235 3.7019806 7.230156 21.33052 16.45932 22.574814c30.325989 4.0885925 61.216156 -1.6049805 91.703415 -4.233597c21.870361 -1.8856659 43.41594 7.994751 65.36745 7.994751",
		"m61.133858 190.4596c14.410118 -0.9737854 29.280548 -5.961746 43.26509 -2.3516998c11.184219 2.8871613 21.443314 9.520508 32.921257 10.816269c22.413116 2.5302582 45.28653 2.3772125 67.24672 7.524933c11.842621 2.7760468 21.626816 5.8235016 29.157486 6.1128693c13.792221 0.529953 29.040268 -7.58667 41.383194 -1.4094543",
		"m66.30708 227.61273c46.126076 -8.542572 94.69835 5.857666 139.2021 20.690277c8.912521 2.9704437 16.024826 -11.53244 25.393707 -12.225723c14.25708 -1.0549774 28.499207 2.8215332 42.795258 2.8215332",
		"m61.133858 262.41208c9.028687 12.403412 31.701561 12.572601 45.6168 6.112854c10.123177 -4.699402 17.996696 -16.45932 29.157486 -16.45932c9.0716095 0 18.350845 1.4131927 26.80577 4.7008057c6.3217316 2.4580994 4.523987 16.401917 11.286087 16.931732c32.745956 2.565796 66.42221 0.9182739 98.2861 -7.0551147"]

	PATHS_CUT = [
		"m61.6063,94.05481c10.847099,-2.326851 19.528023,12.06249 30.566925,13.16535 24.493225,2.44705 49.259255,-2.85347 73.832015,-1.40944m31.50919,15.51968c25.01134,6.10133 50.6969,9.06036 76.18372,12.69554",
		"m65.367455,130.26443c24.469223,-11.1235 52.597985,13.95001 79.475055,13.63779m28.62511,-2.34009c2.00485,-0.20692 4.0081,-0.40283 6.00753,-0.57505 12.79067,-1.10178 25.42493,-1.23466 37.32012,2.91514 18.92751,6.60312 43.66769,30.63415 57.84252,16.45932",
		"m62.076115,163.18391c13.594257,-1.78246 28.332623,-9.68385 40.913385,-4.2336 8.54524,3.70198 7.23016,21.33053 16.45932,22.57482 17.9474,2.41969 36.0924,1.41322 54.23454,-0.35645m37.46888,-3.87715c21.87036,-1.88567 43.41594,7.99475 65.36745,7.99475",
		"m61.133858,190.4596c14.410118,-0.97379 29.280548,-5.96175 43.265092,-2.3517m32.92125,10.81627c22.41312,2.53026 45.28653,2.37721 67.24672,7.52493 11.84263,2.77605 21.62682,5.8235 29.15749,6.11287 13.79222,0.52995 29.04027,-7.58667 41.38319,-1.40945",
		"m66.30708,227.61273c46.12608,-8.54257 94.69835,5.85767 139.2021,20.69028m25.39371,-12.22573c14.25708,-1.05497 28.4992,2.82154 42.79525,2.82154",
		"m61.133858,262.41208c9.028687,12.40341 31.701561,12.5726 45.616802,6.11285 10.12317,-4.6994 17.99669,-16.45932 29.15748,-16.45932m26.80577,4.70081c6.32174,2.4581 4.52399,16.40192 11.28609,16.93173 32.74596,2.5658 66.42221,0.91828 98.2861,-7.05511"]

	def __init__(self, bomb, ident):
		super().__init__(bomb, ident)
		wire_count = random.randint(3, 8)
		if wire_count > 6: wire_count = 6
		self.positions = sorted(random.sample(range(6), wire_count))
		self.cut = [False] * wire_count
		self.colors = []
		for _ in range(wire_count):
			self.colors.append(random.choice(list(Wires.COLORS.keys())))
		self.log(f"There are {len(self.colors)} wires: {' '.join(self.colors)}")

	def get_svg(self):
		svg = '<svg viewBox="0.0 0.0 348.0 348.0" fill="#ffffff" stroke="none" stroke-linecap="square" stroke-miterlimit="10">'
		svg += '<path stroke="#000000" stroke-width="2.0" stroke-linejoin="round" stroke-linecap="butt" d="m5.07874 5.7758217l336.9134 0l0 337.66928l-336.9134 0z" fill-rule="nonzero" />'
		svg += '<path stroke="#000000" stroke-width="2.0" stroke-linejoin="round" stroke-linecap="butt" d="m47.026245 61.745407l29.16536 0l0 225.73228l-29.16536 0z" fill-rule="nonzero" />'
		svg += '<path stroke="#000000" stroke-width="2.0" stroke-linejoin="round" stroke-linecap="butt" d="m257.56955 106.58793l29.165344 0l0 178.20474l-29.165344 0z" fill-rule="nonzero" />'
		svg += '<path stroke="#000000" stroke-width="2.0" stroke-linejoin="round" stroke-linecap="butt" d="m282.73444 40.553925l0 0c0 -8.375591 6.966034 -15.165352 15.5590515 -15.165352l0 0c4.126526 0 8.084015 1.5977726 11.001923 4.441828c2.9178772 2.844057 4.557129 6.7014217 4.557129 10.723524l0 0c0 8.375595 -6.9660034 15.165356 -15.5590515 15.165356l0 0c-8.593018 0 -15.5590515 -6.7897606 -15.5590515 -15.165356z" fill-rule="nonzero" {:s} />'.format(' fill="#00ff00"' if self.solved else '')
		for pos, color, cut in zip(self.positions, self.colors, self.cut):
			svg += '<path stroke="{:s}" stroke-width="6.0" stroke-linejoin="round" stroke-linecap="butt" d="{:s}" />'.format(Wires.COLORS[color],
				(Wires.PATHS_CUT if cut else Wires.PATHS_UNCUT)[pos])
		svg += '</svg>'
		return svg

	@modules.check_solve_cmd
	async def cmd_cut(self, author, parts):
		if len(parts) != 1 or not parts[0].isdigit():
			await self.usage(msg)
		elif parts[0] == "0":
			await self.bomb.channel.send(f"{author.mention} Arrays start at 0, but wires start at 1.")
		else:
			wire = int(parts[0]) - 1
			if wire not in range(len(self.colors)):
				await self.bomb.channel.send(f"There are only {len(self.colors)} wires. How on earth am I supposed to cut wire {parts[0]}?")
			else:
				expected = self.get_solution()
				self.log("player's solution: {:d}".format(wire))
				self.log("correct solution: {:d}".format(expected))
				self.cut[wire] = True
				if expected == wire:
					await self.handle_solved(author)
				else:
					await self.handle_strike(author)

	def get_solution(self):
		def count(color):
			return self.colors.count(color)

		def first(color):
			return self.colors.index(color)

		def last(color):
			return len(self.colors) - 1 - self.colors[::-1].index(color)

		if self.bomb.hummus:
			serial_letter = self.bomb.serial[0].isalpha()

			if serial_letter:
				self.log('the serial number starts with a letter')
			else:
				self.log('the serial number does not start with a letter')

			if len(self.colors) == 3:
				if count("white") == 0 and serial_letter:
					self.log('rule: there are no white wires and the serial number starts with a letter')
					return 1
				elif count("red") == 1:
					self.log('rule: there is exactly one red wire')
					return 0
				elif count("blue") > 1:
					self.log('rule: there is more than one blue wire')
					return first("blue")
				elif self.colors[-1] == "red":
					self.log('rule: the last wire is red')
					return 2
				else:
					self.log('rule: wildcard')
					return 1
			elif len(self.colors) == 4:
				if count("yellow") == 1 and self.colors[-1] == "red":
					self.log('rule: there is exactly one yellow wire and the last wire is red')
					return 2
				elif self.colors[-1] == "white":
					self.log('rule: the last wire is white')
					return 1
				elif count("yellow") == 0:
					self.log('rule: there are no yellow wires')
					return 0
				else:
					self.log('rule: wildcard')
					return 3
			elif len(self.colors) == 5:
				if count("black") > 1 and serial_letter:
					self.log('rule: there is more than one black wire and the serial number starts with a letter')
					return 1
				elif self.colors[-1] == "blue" and count("red") == 1:
					self.log('rule: the last wire is blue and there is exactly one red wire')
					return 0
				elif self.colors[-1] == "red":
					self.log('rule: the last wire is red')
					return 3
				elif count("red") == 0:
					self.log('rule: there are no red wires')
					return 2
				else:
					self.log('rule: wildcard')
					return 0
			elif len(self.colors) == 6:
				if count("red") == 1:
					self.log('rule: there is exactly one red wire')
					return first("red")
				elif self.colors[-1] == "red":
					self.log('rule: the last wire is red')
					return 5
				elif count("yellow") == 0:
					self.log('rule: there are no yellow wires')
					return 3
				else:
					self.log('rule: wildcard')
					return 1
		else:
			serial_odd = int(self.bomb.serial[-1]) % 2 == 1
			self.log('the last digit of the serial number is {:s}'.format('odd' if serial_odd else 'even'))

			if len(self.colors) == 3:
				if count("red") == 0:
					self.log('rule: there are no red wires')
					return 1
				elif self.colors[-1] == "white":
					self.log('rule: the last wire is white')
					return 2
				elif count("blue") > 1:
					self.log('rule: there is more than one blue wire')
					return last("blue")
				else:
					self.log('rule: wildcard')
					return 2
			elif len(self.colors) == 4:
				if count("red") > 1 and serial_odd:
					self.log('rule: there is more than one red wire')
					return last("red")
				elif self.colors[-1] == "yellow" and count("red") == 0:
					self.log('rule: the last wire is yellow and there are no red wires')
					return 0
				elif count("blue") == 1:
					self.log('rule: there is exactly one blue wire')
					return 0
				elif count("yellow") > 1:
					self.log('rule: there is more than one yellow wire')
					return 3
				else:
					self.log('rule: wildcard')
					return 1
			elif len(self.colors) == 5:
				if self.colors[-1] == "black" and serial_odd:
					self.log('rule: the last wire is black and the last digit of the serial number is odd')
					return 3
				elif count("red") == 1 and count("yellow") > 1:
					self.log('rule: there is exactly one red wire and there is more than one yellow wire')
					return 0
				elif count("black") == 0:
					self.log('rule: there are no black wires')
					return 1
				else:
					self.log('rule: wildcard')
					return 0
			else:
				if count("yellow") == 0 and serial_odd:
					self.log('rule: there are no yellow wires and the last digit of the serial number is odd')
					return 2
				elif count("yellow") == 1 and count("white") > 1:
					self.log('rule: there is exactly one yellow wire and there is more than one white wire')
					return 3
				elif count("red") == 0:
					self.log('rule: there are no red wires')
					return 5
				else:
					self.log('rule: wildcard')
					return 3
	
	COMMANDS = {**modules.Module.COMMANDS,
		"cut": cmd_cut
	}
