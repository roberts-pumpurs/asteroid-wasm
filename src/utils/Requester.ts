import axios, { AxiosError } from 'axios';
import { Country, Game, User } from 'types';
import { GameListing, GameListingResponse, Leaderboard } from 'utils/Responses';
import { SaveGameRequestParam } from './Requests';

const apiClient = axios.create();

const urls = {
  games: '/api/games',
  users: '/api/users',
  leaderboards: '/api/leaderboards',
  country: 'http://ip-api.com/json/?fields=countryCode,country', // External API for getting the users location via IP
};

/* ---- Basic HTTP method wrappers ----  */
async function get<T, B>(url: string, params: B): Promise<T> {
  const response = await apiClient
    .get<T>(url, { params })
    .catch((err) => {
      throw err;
    });
  return response.data;
}

async function post<T, B>(url: string, params: B): Promise<T> {
  const response = await apiClient.post<T>(url, params).catch((err) => {
    throw err;
  });
  return response.data;
}

async function put<T, B>(url: string, params: B): Promise<T> {
  const response = await apiClient.put<T>(url, params).catch((err) => {
    throw err;
  });
  return response.data;
}

async function del<T, B>(url: string, params: B): Promise<T> {
  const response = await apiClient.delete<T>(url, params).catch((err) => {
    throw err;
  });
  return response.data;
}

/* ---- Base global accessor for communicating with the backend ---- */
class Requester {
  FetchGames = (): Promise<GameListingResponse> => get(urls.games, {});

  FetchLeaderboards = (): Promise<Leaderboard> => get(urls.leaderboards, {});

  SaveGame = (params: SaveGameRequestParam): Promise<{ created: Game | null }> => post(urls.games, params);

  getCountry = (): Promise<Country> => get(urls.country, {});

  getAllUsers = (): Promise<{ users: Array<User> }> => get(urls.users, {});

  deleteUser = (username: string): Promise<void> => del(`${urls.users}/${username}`, {});

  updateUser = (username: string, user: User): Promise<void> => put(`${urls.users}/${username}`, user);
}

const requester = new Requester();
export { requester as Requester };
