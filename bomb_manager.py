from modules.wires import Wires
from modules.the_button import TheButton

VANILLA_MODULES = {
	"wires": Wires
}

MODDED_MODULES = {
	"theButton": TheButton
}

bombs = {}

# given a function, wrap it around a "bomb must be present" check
def bomb_present(func):
	async def wrapper(msg, parts):
		if msg.channel.id in bombs:
			func(msg, parts)
		else:
			await msg.channel.send("{:s} No bomb is currently running in this channel.".format(msg.author.mention))
	return wrapper

@bomb_present
async def cmd_edgework(msg, parts):
	await msg.channel.send("{:s} Edgework: `{:s}`".format(msg.author.mention, bombs[msg.channel.id].get_edgework()))

@bomb_present
async def cmd_unclaimed(msg, parts):
	modules = [module for module in bombs[msg.channel.id].modules if not module.claim]

	if len(modules) > MAX_UNCLAIMED_LIST_SIZE:
		reply = 'A random sample of unclaimed modules:'
		modules = random.sample(modules, MAX_UNCLAIMED_LIST_SIZE)
	else:
		reply = 'Unclaimed modules:'

	modules.sort(key=lambda module: module.ident)

	for module in modules:
		if module.solved:
			status = 'solved'
		elif module.claim:
			status = 'claimed by {:s}'.format(module.claim)
		else:
			status = 'unclaimed'
		reply += "\n#{:d}: {:s} - {:s}".format(module.ident, module.display_name, status)
	await msg.channel.send(reply)

@bomb_present
async def cmd_claims(msg, parts):
	claims = ['{:s} (#{:d})'.format(module.display_name, module.ident) for module in bombs[msg.channel.id].get_claims(msg.author)]
	if len(claims) == 0:
		await msg.channel.send("{:s} You have not claimed any modules.".format(msg.author.mention))
	elif len(claims) == 1:
		await msg.channel.send("{:s} You have only claimed {:s}.".format(msg.author.mention, claims[0]))
	else:
		await msg.channel.send("{:s} You have claimed {:s} and {:s}.".format(msg.author.mention, ', '.join(claims[:-1]), claims[-1]))

@bomb_present
def handle_module_command(msg, parts):
	pass

async def cmd_modules(msg, parts):
	await msg.channel.send("Available modules:\nVanilla: `" + '`, `'.join(VANILLA_MODULES.keys()) + "`\nModded: `" + '`, `'.join(MODDED_MODULES.keys()) + '`')

async def cmd_run(msg, parts):
	if msg.channel.id in bombs:
		await msg.channel.send("{:s} A bomb is already running in this channel!".format(msg.author.mention))
		return

	usage =("{:s} Usage: `{prefix}run [hummus] <module count> <module distributon> [-<module 1> [-<module 2> [...]]]` or " +
		"`{prefix}run [hummus] <module 1> [<module 2> [...]]`.\n" +
		"For example:\n - `{prefix}run hummus 7 vanilla` - 7 vanilla modules that use the modified manual by LtHummus\n" +
		" - `{prefix}run 12 mixed -souvenir -forgetEverything -forgetMeNot` - 12 modules, each having a 50% chance of " +
		"being a vanilla module. Souvenir, Forget Everything and Forget Me Not modules will not be generated\n" +
		" - `{prefix}run marbleTumble` - a single Marble Tumble module and nothing else\n" +
		" - `{prefix}run hummus complicatedWires morseCode` - two modules: Complicated Wires and Morse Code. Both use LtHummus's manual\n" +
		"Use `!modules` to see the implemented modules.\nAvailable distributions:")

	distributions = {
		"vanilla": 1,
		"mods": 0,
		"mixed": 0.5,
		"lightmixed": 0.67,
		"mixedlight": 0.67,
		"heavymixed": 0.33,
		"mixedheavy": 0.33,
		"light": 0.8,
		"heavy": 0.2,
		"extralight": 0.9,
		"extraheavy": 0.1
	}

	for distribution in distributions:
		if distribution not in ["lightmixed", "heavymixed"]:
			vanilla = int(distributions[distribution] * 100)
			usage += "\n`{:s}`: {:d}% vanilla, {:d}% modded".format(distribution, vanilla, 100 - vanilla)

	if len(parts) < 1:
		await msg.channel.send(usage.format(msg.author.mention, prefix=PREFIX))
		return

	hummus = parts[0] == "hummus"
	if hummus: parts.pop(0)

	if parts[0].isdigit():
		if len(parts) < 2 or parts[1] not in distributions:
			await msg.channel.send(usage.format(msg.author.mention, prefix=PREFIX))
			return

		module_candidates_vanilla = VANILLA_MODULES
		module_candidates_modded = MODDED_MODULES
		module_count = int(parts[0])

		if module_count == 0:
			await msg.channel.send("{:s} What would it even mean for a bomb to have no modules? :thinking:".format(msg.author.mention))
			return

		for veto in parts[2:]:
			if not veto.startswith('-'):
				await msg.channel.send(usage.format(msg.author.mention, prefix=PREFIX))
				return

			veto = veto[1:]
			
			if veto in module_candidates_vanilla:
				del module_candidates_vanilla[veto]
			elif veto in module_candidates_modded:
				del module_candidates_modded[veto]
			else:
				await msg.channel.send("{:s} No such module: `{:s}`".format(msg.author.mention, veto))
				return

		modules = []

		for _ in range(module_count):
			if random.random() < distributions[parts[1]]:
				modules.append(random.choice(list(module_candidates_vanilla.values())))
			else:
				modules.append(random.choice(list(module_candidates_modded.values())))
	else:
		modules = []
		module_candidates = VANILLA_MODULES + MODDED_MODULES
		for module in parts:
			if module not in module_candidates:
				await msg.channel.send("{:s} No such module: `{:s}`".format(msg.author.mention, module))
				return
			modules.append(module_candidates[module])

	bomb = Bomb(modules, hummus)
	bombs[msg.channel.id] = bomb
	await msg.channel.send("A bomb with {:d} modules has been armed!\nEdgework: `{:s}`".format(len(bomb.modules), bomb.get_edgework()))
