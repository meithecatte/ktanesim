from modules.base import Module, noparts, check_solve_cmd, gif_append, gif_output
import glob
from os.path import join as pjoin
from os.path import dirname as pdir
from os.path import basename as pbase
import importlib

VANILLA_MODULE_LIST="wires/button/keypad/simonSays/whosOnFirst/memory/morseCode/complicatedWires/wireSequence/maze/password".split("/")

VANILLA_MODULES={}
MODDED_MODULES={}

for i in glob.glob(pjoin(pdir(__file__),"*.py")):
    if pbase(i) in ["__init__.py","base.py"]:
        continue
    moduleName=pbase(i)[:-3]
    moduleImport=importlib.import_module('modules.'+moduleName)
    classObject=moduleImport.__module_class__
    if moduleName in VANILLA_MODULE_LIST:
        VANILLA_MODULES[moduleName]=classObject
    else:
        MODDED_MODULES[moduleName]=classObject

print("VANILLA:\n",VANILLA_MODULES)
print("MODDED:\n",MODDED_MODULES)

async def cmd_modules(channel, author, parts):
    list_ = lambda d: ', '.join(f"`{x}`" for x in d)
    await channel.send(f"Available modules:\nVanilla: {list_(VANILLA_MODULES)}\nModded: {list_(MODDED_MODULES)}")
