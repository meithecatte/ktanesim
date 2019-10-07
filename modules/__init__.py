from modules.base import Module, noparts, check_solve_cmd, gif_append, gif_output
import glob
from os.path import join as pjoin
from os.path import dirname as pdir
from os.path import basename as pbase
import importlib

VANILLA_MODULES={}
MODDED_MODULES={}

def __find_module_class(moduleName):
    outputInstance=None
    moduleImport=importlib.import_module('modules.'+moduleName)
    attributeList=dir(moduleImport)
    # Loop through each attribute
    for attrName in attributeList:
        if attrName.lower()!=moduleName.lower():
            continue
        testAttr=getattr(moduleImport,attrName)
        # Check if the attribute is an instance of the module class
        if isinstance(testAttr,Module.__class__):
            # if the module is in the list raise an exception as only one Module class should be found for each script
            if not outputInstance is None:
                raise Exception(f"Tried to load multiple classes for one module from the script `{moduleName}.py`")
            outputInstance=testAttr
    if outputInstance is None:
        raise Exception(f"Module was not found in the script `{moduleName}.py`")
    return outputInstance

# Grab all scripts from the current folder
for moduleScript in glob.glob(pjoin(pdir(__file__),"*.py")):
    # Ignore non-module scripts
    if pbase(moduleScript) in ["__init__.py","base.py"]:
        continue

    # Grab the module name an imported instance of it and its attribute list
    moduleName=pbase(moduleScript)[:-3]
    moduleInstance=__find_module_class(moduleName)

    if moduleInstance.vanilla:
        VANILLA_MODULES[moduleName]=moduleInstance
    else:
        MODDED_MODULES[moduleName]=moduleInstance

async def cmd_modules(channel, author, parts):
    list_ = lambda d: ', '.join(f"`{x}`" for x in d)
    await channel.send(f"Available modules:\nVanilla: {list_(VANILLA_MODULES)}\nModded: {list_(MODDED_MODULES)}")
