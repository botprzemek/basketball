interface Game {
  id: string;
  home_name: string;
  away_name: string;
  score: string;
  status: string;
  created_at: number;
}

export default defineEventHandler(async (_event) => {
  const response = await $fetch<Game[]>("http://localhost:3001/api/v1/games");

  return response;
});
