import random
import modules

class WhosOnFirst(modules.Module):
    identifiers = ['whosOnFirst']
    display_name = "Who's on First"
    manual_name = "Who\u2019s on First"
    help_text = "`{cmd} push you're` or `{cmd} press press` to push a button. The phrase must match exactly."
    module_score = 4
    third_base = False
    vanilla = True

    BUTTON_GROUPS = [
        ["READY", "FIRST", "NO", "BLANK", "NOTHING", "YES", "WHAT", "UHHH", "LEFT", "RIGHT", "MIDDLE", "OKAY", "WAIT", "PRESS"],
        ["YOU", "YOU ARE", "YOUR", "YOU'RE", "UR", "U", "UH HUH", "UH UH", "WHAT?", "DONE", "NEXT", "HOLD", "SURE", "LIKE"],
    ]

    DISPLAY_WORDS = {
        "YES": 2, "FIRST": 1, "DISPLAY": 5, "OKAY": 1, "SAYS": 5, "NOTHING": 2,
        "": 4, "BLANK": 3, "NO": 5, "LED": 2, "LEAD": 5, "READ": 3,
        "RED": 3, "REED": 4, "LEED": 4, "HOLD ON": 5, "YOU": 3, "YOU ARE": 5,
        "YOUR": 3, "YOU'RE": 3, "UR": 0, "THERE": 5, "THEY'RE": 4, "THEIR": 3,
        "THEY ARE": 2, "SEE": 5, "C": 1, "CEE": 5
    }

    PRECEDENCE = {
        "READY":   ["YES", "OKAY", "WHAT", "MIDDLE", "LEFT", "PRESS", "RIGHT", "BLANK", "READY", "NO", "FIRST", "UHHH", "NOTHING", "WAIT"],
        "FIRST":   ["LEFT", "OKAY", "YES", "MIDDLE", "NO", "RIGHT", "NOTHING", "UHHH", "WAIT", "READY", "BLANK", "WHAT", "PRESS", "FIRST"],
        "NO":      ["BLANK", "UHHH", "WAIT", "FIRST", "WHAT", "READY", "RIGHT", "YES", "NOTHING", "LEFT", "PRESS", "OKAY", "NO", "MIDDLE"],
        "BLANK":   ["WAIT", "RIGHT", "OKAY", "MIDDLE", "BLANK", "PRESS", "READY", "NOTHING", "NO", "WHAT", "LEFT", "UHHH", "YES", "FIRST"],
        "NOTHING": ["UHHH", "RIGHT", "OKAY", "MIDDLE", "YES", "BLANK", "NO", "PRESS", "LEFT", "WHAT", "WAIT", "FIRST", "NOTHING", "READY"],
        "YES":     ["OKAY", "RIGHT", "UHHH", "MIDDLE", "FIRST", "WHAT", "PRESS", "READY", "NOTHING", "YES", "LEFT", "BLANK", "NO", "WAIT"],
        "WHAT":    ["UHHH", "WHAT", "LEFT", "NOTHING", "READY", "BLANK", "MIDDLE", "NO", "OKAY", "FIRST", "WAIT", "YES", "PRESS", "RIGHT"],
        "UHHH":    ["READY", "NOTHING", "LEFT", "WHAT", "OKAY", "YES", "RIGHT", "NO", "PRESS", "BLANK", "UHHH", "MIDDLE", "WAIT", "FIRST"],
        "LEFT":    ["RIGHT", "LEFT", "FIRST", "NO", "MIDDLE", "YES", "BLANK", "WHAT", "UHHH", "WAIT", "PRESS", "READY", "OKAY", "NOTHING"],
        "RIGHT":   ["YES", "NOTHING", "READY", "PRESS", "NO", "WAIT", "WHAT", "RIGHT", "MIDDLE", "LEFT", "UHHH", "BLANK", "OKAY", "FIRST"],
        "MIDDLE":  ["BLANK", "READY", "OKAY", "WHAT", "NOTHING", "PRESS", "NO", "WAIT", "LEFT", "MIDDLE", "RIGHT", "FIRST", "UHHH", "YES"],
        "OKAY":    ["MIDDLE", "NO", "FIRST", "YES", "UHHH", "NOTHING", "WAIT", "OKAY", "LEFT", "READY", "BLANK", "PRESS", "WHAT", "RIGHT"],
        "WAIT":    ["UHHH", "NO", "BLANK", "OKAY", "YES", "LEFT", "FIRST", "PRESS", "WHAT", "WAIT", "NOTHING", "READY", "RIGHT", "MIDDLE"],
        "PRESS":   ["RIGHT", "MIDDLE", "YES", "READY", "PRESS", "OKAY", "NOTHING", "UHHH", "BLANK", "LEFT", "FIRST", "WHAT", "NO", "WAIT"],
        "YOU":     ["SURE", "YOU ARE", "YOUR", "YOU'RE", "NEXT", "UH HUH", "UR", "HOLD", "WHAT?", "YOU", "UH UH", "LIKE", "DONE", "U"],
        "YOU ARE": ["YOUR", "NEXT", "LIKE", "UH HUH", "WHAT?", "DONE", "UH UH", "HOLD", "YOU", "U", "YOU'RE", "SURE", "UR", "YOU ARE"],
        "YOUR":    ["UH UH", "YOU ARE", "UH HUH", "YOUR", "NEXT", "UR", "SURE", "U", "YOU'RE", "YOU", "WHAT?", "HOLD", "LIKE", "DONE"],
        "YOU'RE":  ["YOU", "YOU'RE", "UR", "NEXT", "UH UH", "YOU ARE", "U", "YOUR", "WHAT?", "UH HUH", "SURE", "DONE", "LIKE", "HOLD"],
        "UR":      ["DONE", "U", "UR", "UH HUH", "WHAT?", "SURE", "YOUR", "HOLD", "YOU'RE", "LIKE", "NEXT", "UH UH", "YOU ARE", "YOU"],
        "U":       ["UH HUH", "SURE", "NEXT", "WHAT?", "YOU'RE", "UR", "UH UH", "DONE", "U", "YOU", "LIKE", "HOLD", "YOU ARE", "YOUR"],
        "UH HUH":  ["UH HUH", "YOUR", "YOU ARE", "YOU", "DONE", "HOLD", "UH UH", "NEXT", "SURE", "LIKE", "YOU'RE", "UR", "U", "WHAT?"],
        "UH UH":   ["UR", "U", "YOU ARE", "YOU'RE", "NEXT", "UH UH", "DONE", "YOU", "UH HUH", "LIKE", "YOUR", "SURE", "HOLD", "WHAT?"],
        "WHAT?":   ["YOU", "HOLD", "YOU'RE", "YOUR", "U", "DONE", "UH UH", "LIKE", "YOU ARE", "UH HUH", "UR", "NEXT", "WHAT?", "SURE"],
        "DONE":    ["SURE", "UH HUH", "NEXT", "WHAT?", "YOUR", "UR", "YOU'RE", "HOLD", "LIKE", "YOU", "U", "YOU ARE", "UH UH", "DONE"],
        "NEXT":    ["WHAT?", "UH HUH", "UH UH", "YOUR", "HOLD", "SURE", "NEXT", "LIKE", "DONE", "YOU ARE", "UR", "YOU'RE", "U", "YOU"],
        "HOLD":    ["YOU ARE", "U", "DONE", "UH UH", "YOU", "UR", "SURE", "WHAT?", "YOU'RE", "NEXT", "HOLD", "UH HUH", "YOUR", "LIKE"],
        "SURE":    ["YOU ARE", "DONE", "LIKE", "YOU'RE", "YOU", "HOLD", "UH HUH", "UR", "SURE", "U", "WHAT?", "NEXT", "YOUR", "UH UH"],
        "LIKE":    ["YOU'RE", "NEXT", "U", "UR", "HOLD", "DONE", "UH UH", "WHAT?", "UH HUH", "YOU", "LIKE", "SURE", "YOU ARE", "YOUR"],
    }

    def __init__(self, bomb, ident):
        super().__init__(bomb, ident)
        self.stage = 0
        self.randomize()

    def get_svg(self, led):
        transform = 'rotate(180 174 174)' if self.third_base else 'none'
        svg = (
            f'<svg viewBox="0 0 348 348" fill="#fff" stroke-linecap="butt" stroke-linejoin="round" stroke-miterlimit="10" transform="{transform}">'
            f'<path stroke="#000" stroke-width="2" d="M5 5h338v338h-338z"/>'
            f'<circle fill="{led}" stroke="#000" cx="298" cy="40.5" r="15" stroke-width="2"/>'
            '<path fill="#000" stroke="#000" stroke-width="2" d="M34 25h230v67h-232zM277 106h52v208h-52z"/>'
            '<path stroke="#000" d="M34 125h106v44h-106zM158 125h106v44h-106zM34 202h106v44h-106zM158 202h106v44h-106zM34 270h106v44h-106zM158 270h106v44h-106z"/>' +
            '<text x="149" y="72" text-anchor="middle" style="font-family:sans-serif;font-size:28pt;">{:s}</text>'.format(self.display))

        for i in range(3):
            color = '#0f0' if self.stage > i else '#fff'
            svg += f'<path fill="{color}" stroke="{color}" stroke-width="2" d="M288 {257 - 57 * i}h30v20h-30z"/>'

        for index, text in enumerate(self.buttons):
            x = [87, 211][index % 2]
            y = [155, 232, 300][index // 2]
            svg += f'<text x="{x}" y="{y}" text-anchor="middle" style="font-family:sans-serif;font-size:16pt;" fill="#000">{text}</text>'
        svg += '</svg>'
        return svg

    def randomize(self):
        self.display = random.choice(list(self.DISPLAY_WORDS.keys()))
        self.buttons = random.sample(random.choice(self.BUTTON_GROUPS), 6)
        self.log(f"State randomized. Stage {self.stage}. Display: {self.display}. Buttons: {' '.join(self.buttons)}")

    @modules.check_solve_cmd
    async def cmd_push(self, author, parts):
        if not parts:
            return await self.usage(author)
        button = ' '.join(parts).upper()

        if self.third_base:
            button = button.replace('0', 'O').replace('1', 'I')

        if button not in sum(self.BUTTON_GROUPS, []):
            return await self.bomb.channel.send(f"{author.mention} \"{button}\" isn't a valid word.")

        if button not in self.buttons:
            return await self.handle_unsubmittable(author)

        solution = self.get_solution()

        self.log(f"Pressed {button}, expected {solution}")

        if button == solution:
            self.stage += 1
            if self.stage == 3:
                await self.handle_solve(author)
            else:
                self.randomize()
                await self.handle_next_stage(author)
        else:
            self.randomize()
            await self.handle_strike(author)

    def get_solution(self):
        index = self.DISPLAY_WORDS[self.display]

        word = self.buttons[index]
        self.log(f"Button to look at is {index}, the word is {word}")

        precedence = self.PRECEDENCE[word]

        self.log(f"Precedence list: {','.join(precedence)}")
        for button in precedence:
            if button in self.buttons:
                self.log(f"Button to press is {button}")
                return button
        assert False, "The rules should prevent this"

    COMMANDS = {
        "push": cmd_push,
        "press": cmd_push,
    }

class ThirdBase(WhosOnFirst):
    identifiers = ['thirdBase']
    display_name = "Third Base"
    manual_name = "Third Base"
    help_text = "`{cmd} push 8i99` or `{cmd} press 66i8` to push a button."
    module_score = 6
    third_base = True
    vanilla = False

    DISPLAY_WORDS = {
        "NHXS": 2, "IH6X": 1, "XI8Z": 5, "I8O9": 1, "XOHZ": 5, "H68S": 2,
        "8OXN": 4, "Z8IX": 3, "SXHN": 5, "6NZH": 2, "H6SI": 5, "6O8I": 3,
        "NXO8": 3, "66I8": 4, "S89H": 4, "SNZX": 5, "9NZS": 3, "8I99": 5,
        "ZHOX": 3, "SI9X": 3, "SZN6": 0, "ZSN8": 5, "HZN9": 4, "X9HI": 3,
        "IS9H": 2, "XZNS": 5, "X6IS": 1, "8NSZ": 5,
    }

    PRECEDENCE = {
        "XI8Z": ["NHXS", "I8O9", "XOHZ", "6O8I", "6NZH", "66I8", "H6SI", "Z8IX", "XI8Z", "SXHN", "H68S", "8OXN", "IH6X", "NXO8"],
        "H68S": ["6NZH", "I8O9", "NHXS", "6O8I", "SXHN", "H6SI", "IH6X", "8OXN", "NXO8", "XI8Z", "Z8IX", "XOHZ", "66I8", "H68S"],
        "SXHN": ["Z8IX", "8OXN", "NXO8", "H68S", "XOHZ", "XI8Z", "H6SI", "NHXS", "IH6X", "6NZH", "66I8", "I8O9", "SXHN", "6O8I"],
        "Z8IX": ["NXO8", "H6SI", "I8O9", "6O8I", "Z8IX", "66I8", "XI8Z", "IH6X", "SXHN", "XOHZ", "6NZH", "8OXN", "NHXS", "H68S"],
        "IH6X": ["8OXN", "H6SI", "I8O9", "6O8I", "NHXS", "Z8IX", "SXHN", "66I8", "6NZH", "XOHZ", "NXO8", "H68S", "IH6X", "XI8Z"],
        "NHXS": ["I8O9", "H6SI", "8OXN", "6O8I", "H68S", "XOHZ", "66I8", "XI8Z", "IH6X", "NHXS", "6NZH", "Z8IX", "SXHN", "NXO8"],
        "XOHZ": ["8OXN", "XOHZ", "6NZH", "IH6X", "XI8Z", "Z8IX", "6O8I", "SXHN", "I8O9", "H68S", "NXO8", "NHXS", "66I8", "H6SI"],
        "8OXN": ["XI8Z", "IH6X", "6NZH", "XOHZ", "I8O9", "NHXS", "H6SI", "SXHN", "66I8", "Z8IX", "8OXN", "6O8I", "NXO8", "H68S"],
        "6NZH": ["H6SI", "6NZH", "H68S", "SXHN", "6O8I", "NHXS", "Z8IX", "XOHZ", "8OXN", "NXO8", "66I8", "XI8Z", "I8O9", "IH6X"],
        "H6SI": ["NHXS", "IH6X", "XI8Z", "66I8", "SXHN", "NXO8", "XOHZ", "H6SI", "6O8I", "6NZH", "8OXN", "Z8IX", "I8O9", "H68S"],
        "6O8I": ["Z8IX", "XI8Z", "I8O9", "XOHZ", "IH6X", "66I8", "SXHN", "NXO8", "6NZH", "6O8I", "H6SI", "H68S", "8OXN", "NHXS"],
        "I8O9": ["6O8I", "SXHN", "H68S", "NHXS", "8OXN", "IH6X", "NXO8", "I8O9", "6NZH", "XI8Z", "Z8IX", "66I8", "XOHZ", "H6SI"],
        "NXO8": ["8OXN", "SXHN", "Z8IX", "I8O9", "NHXS", "6NZH", "H68S", "66I8", "XOHZ", "NXO8", "IH6X", "XI8Z", "H6SI", "6O8I"],
        "66I8": ["H6SI", "6O8I", "NHXS", "XI8Z", "66I8", "I8O9", "IH6X", "8OXN", "Z8IX", "6NZH", "H68S", "XOHZ", "SXHN", "NXO8"],
        "9NZS": ["8NSZ", "8I99", "ZHOX", "HZN9", "IS9H", "SNZX", "SZN6", "XZNS", "SI9X", "9NZS", "ZSN8", "X6IS", "X9HI", "S89H"],
        "8I99": ["ZHOX", "IS9H", "X6IS", "SNZX", "SI9X", "X9HI", "ZSN8", "XZNS", "9NZS", "S89H", "HZN9", "8NSZ", "SZN6", "8I99"],
        "ZHOX": ["ZSN8", "8I99", "SNZX", "ZHOX", "IS9H", "SZN6", "8NSZ", "S89H", "HZN9", "9NZS", "SI9X", "XZNS", "X6IS", "X9HI"],
        "HZN9": ["9NZS", "HZN9", "SZN6", "IS9H", "ZSN8", "8I99", "S89H", "ZHOX", "SI9X", "SNZX", "8NSZ", "X9HI", "X6IS", "XZNS"],
        "SZN6": ["X9HI", "S89H", "SZN6", "SNZX", "SI9X", "8NSZ", "ZHOX", "XZNS", "HZN9", "X6IS", "IS9H", "ZSN8", "8I99", "9NZS"],
        "S89H": ["SNZX", "8NSZ", "IS9H", "SI9X", "HZN9", "SZN6", "ZSN8", "X9HI", "S89H", "9NZS", "X6IS", "XZNS", "8I99", "ZHOX"],
        "SNZX": ["SNZX", "ZHOX", "8I99", "9NZS", "X9HI", "XZNS", "ZSN8", "IS9H", "8NSZ", "X6IS", "HZN9", "SZN6", "S89H", "SI9X"],
        "ZSN8": ["SZN6", "S89H", "8I99", "HZN9", "IS9H", "ZSN8", "X9HI", "9NZS", "SNZX", "X6IS", "ZHOX", "8NSZ", "XZNS", "SI9X"],
        "SI9X": ["9NZS", "XZNS", "HZN9", "ZHOX", "S89H", "X9HI", "ZSN8", "X6IS", "8I99", "SNZX", "SZN6", "IS9H", "SI9X", "8NSZ"],
        "X9HI": ["8NSZ", "SNZX", "IS9H", "SI9X", "ZHOX", "SZN6", "HZN9", "XZNS", "X6IS", "9NZS", "S89H", "8I99", "ZSN8", "X9HI"],
        "IS9H": ["SI9X", "SNZX", "ZSN8", "ZHOX", "XZNS", "8NSZ", "IS9H", "X6IS", "X9HI", "8I99", "SZN6", "HZN9", "S89H", "9NZS"],
        "XZNS": ["8I99", "S89H", "X9HI", "ZSN8", "9NZS", "SZN6", "8NSZ", "SI9X", "HZN9", "IS9H", "XZNS", "SNZX", "ZHOX", "X6IS"],
        "8NSZ": ["8I99", "X9HI", "X6IS", "HZN9", "9NZS", "XZNS", "SNZX", "SZN6", "8NSZ", "S89H", "SI9X", "IS9H", "ZHOX", "ZSN8"],
        "X6IS": ["HZN9", "IS9H", "S89H", "SZN6", "XZNS", "X9HI", "ZSN8", "SI9X", "SNZX", "9NZS", "X6IS", "8NSZ", "8I99", "ZHOX"],
    }

    BUTTON_GROUPS = [list(PRECEDENCE)]
