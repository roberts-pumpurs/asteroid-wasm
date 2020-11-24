import axios, { AxiosError } from 'axios';
import { Country, Game, User } from 'types';
import { GameListing, GameListingResponse, Leaderboard } from 'utils/Responses';
import { SaveGameRequestParam } from './Requests';

const apiClient = axios.create({
  xsrfCookieName: 'csrftoken',
  xsrfHeaderName: 'X-CSRFTOKEN',
});

const urls = {
  games: '/api/games',
  users: '/api/users',
  leaderboards: '/api/leaderboards',
  country: 'http://ip-api.com/json/?fields=countryCode,country',
  backendCountry: '/api/countries',
};

function isAxiosError(err: AxiosError | unknown): err is AxiosError {
  return (err as AxiosError).response !== undefined;
}

function handleError(err: AxiosError | unknown, url: string): void {
  if (isAxiosError(err)) {
    console.error(
      "Endpoint: '",
      url,
      "' returned an error\nMessage: ",
      err.message,
    );
  }
}

async function get<T, B>(url: string, params: B): Promise<T> {
  const response = await apiClient
    .get<T>(url, { params })
    .catch((err) => {
      handleError(err, url);
      throw err;
    });
  return response.data;
}

async function post<T, B>(url: string, params: B): Promise<T> {
  const response = await apiClient.post<T>(url, params).catch((err) => {
    handleError(err, url);
    throw err;
  });
  return response.data;
}

async function download<T, B>(url: string, params: B): Promise<T> {
  const response = await apiClient
    .post<T>(url, params, {
    responseType: 'blob',
  })
    .catch((err) => {
      handleError(err, url);
      throw err;
    });
  return response.data;
}


class Requester {
  FetchGames = (): Promise<GameListingResponse> => get(urls.games, {});

  FetchLeaderboards = (): Promise<Leaderboard> => get(urls.leaderboards, {});

  SaveGame = (params: SaveGameRequestParam): Promise<{created: Game | null}> => post(urls.games, params);

  getCountry = (): Promise<Country> => get(urls.country, {});

}

const requester = new Requester();
export { requester as Requester };
