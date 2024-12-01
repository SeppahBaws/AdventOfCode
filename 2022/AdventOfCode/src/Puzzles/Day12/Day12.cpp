#include "pch.h"
#include "Day12.h"

#include <fmt/core.h>
#include "Common/FileUtils.h"

namespace Day12
{
	struct Tile
	{
		u32 x{};
		u32 y{};

		u8 elevation{};

		i32 f{}, g{}, h{};
		Tile* pathParent{};
	};

	constexpr u8 ELEVATION_START = 0;
	constexpr u8 ELEVATION_END = 'z' - 'a' + 2;
	typedef std::vector<std::vector<Tile>> World;

	void ParseWorld(const std::vector<std::string>& lines, World* world, Tile* start, Tile* end)
	{
		delete world;
		world = new World();

		for (const std::string& line : lines)
		{
			std::vector<Tile>& row = world->emplace_back();

			for (char c : line)
			{
				Tile& tile = row.emplace_back(
					world->size() - 1,
					row.size() - 1
				);

				if (c == 'S')
				{
					tile.elevation = ELEVATION_START;
					start = &tile;
				}
				else if (c == 'E')
				{
					tile.elevation = ELEVATION_END;
					end = &tile;
				}
				else
				{
					tile.elevation = c - 'a' + 1;
				}
			}
		}
	}

	std::vector<Tile*> GetNeighbors(Tile* tile, World* world)
	{
		std::vector<Tile*> neighbors{};

		// Left
		if (tile->x > 0)
		{
			neighbors.push_back(&(*world)[tile->y][tile->x - 1]);
		}

		// Right
		if (tile->x + 1 < world[tile->y].size())
		{
			neighbors.push_back(&(*world)[tile->y][tile->x + 1]);
		}

		// Top
		if (tile->y > 0)
		{
			neighbors.push_back(&(*world)[tile->y - 1][tile->x]);
		}

		// Bottom
		if (tile->y + 1 < world->size())
		{
			neighbors.push_back(&(*world)[tile->y + 1][tile->x]);
		}

		return neighbors;
	}

	i32 GetDistance(const Tile* tileA, const Tile* tileB)
	{
		const i32 distX = abs(i32(tileA->x) - i32(tileB->x));
		const i32 distY = abs(i32(tileA->y) - i32(tileB->y));

		if (distX > distY)
			return 14 * distY + 10 * (distX - distY);
		return 14 * distX + 10 * (distY - distX);
	}

	std::vector<Tile*> FindPathAStar(Tile* start, Tile* end, World* world)
	{
		std::vector<Tile*> openSet{};
		std::vector<Tile*> closedSet{};
		openSet.push_back(start);

		while (!openSet.empty())
		{
			// Find the tile with the lowest f-value
			Tile* q = openSet.front();
			for (Tile* tile : openSet)
			{
				if (tile->f < q->f)
				{
					q = tile;
				}
			}

			std::erase(openSet, q);
			closedSet.push_back(q);

			if (q == end)
			{
				return closedSet;
			}

			for (Tile* neighbor : GetNeighbors(q, world))
			{
				if (abs(neighbor->elevation - q->elevation) > 1 || std::ranges::find(closedSet, neighbor) != closedSet.end())
				{
					continue;
				}

				const i32 newCostToNeighbor = q->g + GetDistance(q, neighbor); // Hard-coded distance
				if (newCostToNeighbor < neighbor->g || std::ranges::find(openSet, neighbor) == openSet.end())
				{
					neighbor->g = newCostToNeighbor;
					neighbor->h = GetDistance(neighbor, end);
					neighbor->pathParent = q;

					if (std::ranges::find(openSet, neighbor) == openSet.end())
						openSet.push_back(neighbor);
				}
			}
		}

		return {};
	}

	void Run()
	{
		fmt::print("=====[ AdventOfCode Day 12 ]=====\n");

		std::vector<std::string> lines{};
		Common::ReadFileSync("src/Puzzles/Day12/input.txt", lines);

		Tile* startTile{};
		Tile* endTile{};
		World* world{};
		ParseWorld(lines, world, startTile, endTile);

		fmt::print("\n-----[ Part 1 ]-----\n");

		fmt::print("Going from ({}, {}) to ({}, {})\n", startTile->x, startTile->y, endTile->x, endTile->y);

		std::vector<Tile*> path = FindPathAStar(startTile, endTile, world);


		fmt::print("\n-----[ Part 2 ]-----\n");
	}
}
