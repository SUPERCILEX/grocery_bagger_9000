import random

from bag_filler import *


class BuildingBlock():
    def __init__(self, pieces=None):
        if pieces == None:
            pieces = []
        self.pieces = pieces

    @classmethod
    def from_bag(cls, bag):
        return cls(bag.pieces);

    def new(self):
        pieces = []
        for p in self.pieces:
            pieces.append(str(p))
        return BuildingBlock(pieces)

    def pop(self):
        if len(self.pieces) > 0:
            piece = self.pieces.pop(0)
            return (True, piece)
        return (False, "")

    def len(self):
        return len(self.pieces)

    def is_empty(self):
        return len(self.pieces) > 0


class LevelGenerator():
    def __init__(self, sizes=((3, 4)), colors=("Red", "Gold", "Pink", "Blue", "Green")):
        self.sizes = sizes
        self.building_blocks = []
        self.colors = colors
        for size in self.sizes:
            ##            print(size)
            ##            print(self.sizes)
            bags = GetUniqueBagFillings(size[0], size[1])
            blocks = []
            for bag in bags:
                blocks.append(BuildingBlock.from_bag(bag))
            self.building_blocks.append((blocks))

    def get_building_block(self, index):
        return random.choice(self.building_blocks[index]).new()

    def get_random_color(self):
        return str(random.choice(self.colors))

    def level_insert(self, level=None, nomino=(LevelPiece("piece", "color")), num_selectable=3):
        if level == None:
            level = []
        level_len = len(level)
        index = 0
        if level_len == 0:
            level.insert(0, nomino)
        elif level_len < num_selectable:
            index = random.randrange(0, level_len)
        else:
            index = random.randrange(0, num_selectable)
        level.insert(index, (nomino))
        return level

    def generate_level(self, sizes=(0, 0, 0), min_pieces=12, ):
        level = []
        blocks = []
        pieces = 0
        colors = []
        for i in sizes:
            block = self.get_building_block(i)
            pieces += block.len()
            blocks.append(block)
            colors.append(self.get_random_color())
        while len(blocks) > 0:
            ##            print("Pieces: " + str(pieces) + "\nlen(blocks): " + str(len(blocks)))
            index = random.randrange(0, len(blocks))
            size_key = sizes[index]
            ##            print("Index: " + str(index))
            result = blocks[index].pop()
            ##            print("Result: " + str(result))
            if result[0]:
                ##                print(level)#=
                level = self.level_insert(level, LevelPiece(result[1], colors[index]))
            ##                print(level)
            ##                level.append((result[1], colors[index]))

            ##            print(level)
            ##            print(blocks[index])
            ##            print(blocks[index].is_empty())
            if blocks[index].len() == 0:
                ##                print("blocks is empty")
                if pieces >= min_pieces:
                    ##                    print("enough pieces")
                    blocks.pop(index)
                else:
                    ##                    print("get another block")
                    blocks.pop(index)
                    blocks.insert(index, self.get_building_block(size_key))
                    pieces += blocks[index].len()
                    colors.pop(index)
                    colors.insert(index, self.get_random_color())
        level.reverse()
        return level


def main():
    ##    level_gen = LevelGenerator[(2,4),(3,4),(4,4),(5,4))
    level_gen = LevelGenerator([(3, 4)])
    for i in range(1):
        ##        print("Level " + str(i) + ":\n")
        level = level_gen.generate_level()
        print(*level, sep=',\n')


##        print("done")
##        print("\n")

main()
