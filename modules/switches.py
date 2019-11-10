import random
import modules
import enum

class Switches(modules.Module):
    class D:
        UP = False
        DOWN = True
    identifiers = ['switches']
    display_name = "Switches"
    manual_name = "Switches"
    help_text = "`{cmd} flip 1 5 3 4 2 5 2` or `{cmd} flip 1534252`. Partial solutions allowed."
    module_score = 4

    # each number is a switch in the up state
    invalid_positions = {"3","245","2345","14","145","1345","12","124","123","1234"}
    switch_polygon = [(50.5, 174), (65.5, 174), (73, 234), (43, 234)]

    # lists indexes of switches in the up position
    def up_state_positions(self, switch_list):
        return "".join(str(i + 1) for i, val in enumerate(switch_list) if val == self.D.UP)

    # checks if the state is valid
    def is_valid_state(self, switch_state):
        return not self.up_state_positions(switch_state) in self.invalid_positions

    def state_as_string(self, state):
        return " ".join("UP" if switch == self.D.UP else "DOWN" for switch in state)

    def gen_rand(self):
        return [random.choice([self.D.DOWN, self.D.UP]) for i in range(5)]

    def __init__(self, bomb, ident):
        super().__init__(bomb, ident)

        self.solution = self.gen_rand()
        while not self.is_valid_state(self.solution):
            self.solution = self.gen_rand()

        self.switches = self.gen_rand()
        while True:
            if self.is_valid_state(self.switches) and self.switches != self.solution:
                break
            self.switches = self.gen_rand()

        self.log(f"Initial position: {self.state_as_string(self.switches)}")
        self.log(f"Solution: {self.state_as_string(self.solution)}")

    def generateSwitch(self, index):
        positions = [[x[0] + 58 * index, x[1]] for x in self.switch_polygon]
        if self.switches[index] == self.D.UP:
            positions[2][1] = 348 - positions[2][1]
            positions[3][1] = 348 - positions[3][1]

        positions = " ".join(f"{pos[0]},{pos[1]}" for pos in positions)

        o = f'<circle stroke-width="2" stroke="#000" cx="{58*(index+1)}" cy="174" r="15" />'
        o += f'<circle stroke-width="2" stroke="#000" fill="{"#0f0" if self.solution[index] == self.D.UP else "#fff"}" cx="{58*(index+1)}" cy="87" r="10" />'
        o += f'<circle stroke-width="2" stroke="#000" fill="{"#0f0" if self.solution[index] == self.D.DOWN else "#fff"}" cx="{58*(index+1)}" cy="261" r="10" />'
        o += f'<polygon points="{positions}" style="fill:black;stroke:black;stroke-width:2" />'

        return o

    def get_svg(self, led):
        output = (f'<svg viewBox="0 0 348 348" fill="#fff" stroke-linecap="butt" stroke-linejoin="round" stroke-miterlimit="10" xmlns:xlink="http://www.w3.org/1999/xlink">'
                f'<path stroke="#000" stroke-width="2" d="M5 5h338v338h-338z"/>'
                f'<circle fill="{led}" stroke="#000" cx="298" cy="40.5" r="15" stroke-width="2"/>')

        for i in range(5):
            output += self.generateSwitch(i)
        output += '</svg>'
        return output

    def flip_switch_by_index(self, index):
        self.switches[index] = not self.switches[index]

    @modules.check_solve_cmd
    async def cmd_flip(self, author, parts):
        if not parts:
            return await self.usage(author)

        input_ = []

        for part in parts:
            if part.isdigit():
                for digit in part:
                    switch = int(digit)
                    if switch not in range(1, 6):
                        return await self.bomb.channel.send(f"{author.mention} There is no switch {switch}! There are five switches: 1-5.")
                    input_.append(switch - 1)
            else:
                return await self.bomb.channel.send(f"{author.mention} No such switch: `{part}`")

        while input_ and not self.solved:
            print("New switches state: "+self.state_as_string(self.switches))
            press = input_.pop(0)
            self.flip_switch_by_index(press)
            if not self.is_valid_state(self.switches):
                self.flip_switch_by_index(press)
                await self.handle_strike(author)
                return
            if self.switches == self.solution:
                await self.handle_solve(author)
        if not self.solved:
            await self.do_view(author.mention)

    COMMANDS = {
        "flip": cmd_flip
    }
