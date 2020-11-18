import axios, { AxiosError } from 'axios';
import { GameListing, GameListingResponse, Leaderboard } from 'utils/Responses';

const apiClient = axios.create({
  xsrfCookieName: 'csrftoken',
  xsrfHeaderName: 'X-CSRFTOKEN',
});

const urls = {
  'fetch-games': '/api/games',
  'fetch-leaderboards': '/api/leaderboards',
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
  FetchGames = (): Promise<GameListingResponse> => get(urls['fetch-games'], {});
  FetchLeaderboards = (): Promise<Leaderboard> => get(urls['fetch-leaderboards'], {});
}

const requester = new Requester();
export { requester as Requester };
