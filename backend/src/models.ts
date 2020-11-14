import { DateTime } from "neo4j-driver";

export interface User {
    name: string,
    surname: string,
    username: string,
}

export interface Country {
    name: string,
    population: number,
}

export interface Game {
    score: number,
    start: DateTime,
    end: DateTime,
}
