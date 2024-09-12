import { webGet } from "src/webc";

export interface Ingredient {
  id: number;
  name: string;
  quantity: string;
}

export type IngredientPatch = Partial<Omit<Ingredient, 'id'>>;

class IngredientMco {

  async list(): Promise<Ingredient[]> {
    const data = await webGet("ingredients");
    return data as Ingredient[];
  }
}

export const ingredientMco = new IngredientMco();
