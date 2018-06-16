from modules.base import Module, noparts, check_solve_cmd, gif_append, gif_output
from modules.wires import Wires
from modules.button import Button
from modules.keypad import Keypad
from modules.simon import SimonSays
from modules.whos_on_first import WhosOnFirst
from modules.memory import Memory

VANILLA_MODULES = {
	"wires": Wires,
	"button": Button,
	"keypad": Keypad,
	"simonSays": SimonSays,
	"whosOnFirst": WhosOnFirst,
	"memory": Memory,
}

MODDED_MODULES = {
}

async def cmd_modules(channel, author, parts):
	list_ = lambda d: ', '.join(f"`{x}`" for x in d)
	await channel.send(f"Available modules:\nVanilla: {list_(VANILLA_MODULES)}\nModded: {list_(MODDED_MODULES)}")
