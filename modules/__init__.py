from modules.base import Module, noparts, check_solve_cmd, gif_append, gif_output
import glob
from os.path import join as pjoin
from os.path import dirname as pdir
from os.path import basename as pbase
import importlib

VANILLA_MODULES = {}
MODDED_MODULES = {}


def __find_module_class(module_name):
    output_instance = None
    module_import = importlib.import_module('modules.' + module_name)
    attribute_list = dir(module_import)
    # Loop through each attribute
    for attr_name in attribute_list:
        if attr_name.lower() != module_name.lower():
            continue
        test_attr = getattr(module_import, attr_name)
        # Check if the attribute is an instance of the module class
        if isinstance(test_attr, Module.__class__):
            # if the module is in the list raise an exception as only one Module class should be found for each script
            if not output_instance is None:
                raise Exception(
                    f"Tried to load multiple classes for one module from the script `{module_name}.py`")
            output_instance = test_attr
    if output_instance is None:
        raise Exception(
            f"Module was not found in the script `{module_name}.py`")
    return output_instance


# Grab all scripts from the current folder
for module_script in glob.glob(pjoin(pdir(__file__), "*.py")):
    # Ignore non-module scripts
    if pbase(module_script) in ["__init__.py", "base.py"]:
        continue

    # Grab the module name an imported instance of it and its attribute list
    module_name = pbase(module_script)[:-3]
    module_instance = __find_module_class(module_name)

    if module_instance.vanilla:
        VANILLA_MODULES[module_name] = module_instance
    else:
        MODDED_MODULES[module_name] = module_instance


async def cmd_modules(channel, author, parts):
    def list_(d): return ', '.join(f"`{x}`" for x in d)
    await channel.send(f"Available modules:\nVanilla: {list_(VANILLA_MODULES)}\nModded: {list_(MODDED_MODULES)}")
