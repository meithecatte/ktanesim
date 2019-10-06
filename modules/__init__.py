from modules.base import Module, noparts, check_solve_cmd, gif_append, gif_output
import glob
from os.path import join as pjoin
from os.path import dirname as pdir
from os.path import basename as pbase
import importlib

VANILLA_MODULES={}
MODDED_MODULES={}

for i in glob.glob(pjoin(pdir(__file__),"*.py")):
    if pbase(i) in ["__init__.py","base.py"]:
        continue
    moduleName=pbase(i)[:-3]
    moduleImport=importlib.import_module('modules.'+moduleName)
    classObject=dir(moduleImport)
    for i in classObject:
        j=getattr(moduleImport,i)
        try:
            if j.__mro__[-1]==Module:
                if j.vanilla:
                    VANILLA_MODULES[moduleName]=j
                else:
                    MODDED_MODULES[moduleName]=j
        except:
            pass

async def cmd_modules(channel, author, parts):
    list_ = lambda d: ', '.join(f"`{x}`" for x in d)
    await channel.send(f"Available modules:\nVanilla: {list_(VANILLA_MODULES)}\nModded: {list_(MODDED_MODULES)}")
