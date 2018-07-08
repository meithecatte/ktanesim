from modules.base import Module, noparts, check_solve_cmd, gif_output

from modules.wires import Wires, ComplicatedWires, WireSequence
from modules.button import Button
from modules.keypad import Keypad
from modules.simon import SimonSays
from modules.whos_on_first import WhosOnFirst, ThirdBase
from modules.memory import Memory
from modules.morse import MorseCode
from modules.maze import Maze
from modules.password import Password

from modules.connection_check import ConnectionCheck
from modules.hexamaze import Hexamaze

VANILLA_MODULES = {
	"wires": Wires,
	"button": Button,
	"keypad": Keypad,
	"simonSays": SimonSays,
	"whosOnFirst": WhosOnFirst,
	"memory": Memory,
	"morseCode": MorseCode,
	"complicatedWires": ComplicatedWires,
	"wireSequence": WireSequence,
	"maze": Maze,
	"password": Password,
}

MODDED_MODULES = {
	"connectionCheck": ConnectionCheck,
	"hexamaze": Hexamaze,
	"thirdBase": ThirdBase,
}

async def cmd_modules(channel, author, parts):
	list_ = lambda d: ', '.join(f"`{x}`" for x in d)
	await channel.send(f"Available modules:\nVanilla: {list_(VANILLA_MODULES)}\nModded: {list_(MODDED_MODULES)}")
