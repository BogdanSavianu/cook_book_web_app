import { hub } from "dom-native";
import { webDelete, webGet, webPatch, webPost } from "src/webc";

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

  async create(data: IngredientPatch): Promise<Ingredient> {
    // guard (INGREDIENT - validate data)
    if (data.name == null || data.name.trim().length == 0) {
      throw new Error("Cannot create ingredient with empty name");
    }
    if (data.quantity == null || data.quantity.trim().length == 0) {
      throw new Error("Cannot create ingredient with empty quantity");
    }
    // to server
    const newData = await webPost('ingredients', data);
    // send event
    hub('dataHub').pub('Ingredient', 'create', newData);

    return newData as Ingredient;
  }

  async update(id: number, data: IngredientPatch): Promise<Ingredient> {
    if (data.name == null || data.name.trim().length == 0) {
      throw new Error("Cannot update ingredient with empty name");
    }
    if (data.quantity == null || data.quantity.trim().length == 0) {
      throw new Error("Cannot update ingredient with empty quantity");
    }

    const newData = await webPatch(`ingredients/${id}`, data);
    // event
    hub('dataHub').pub('Ingredient', 'update', newData);

    return newData as Ingredient;
  }

  async delete(id: number): Promise<Ingredient> {
    // to server
    const oldData = await webDelete(`ingredients/${id}`);
    //event
    hub('dataHub').pub('Ingredient', 'delete', oldData);

    return oldData as Ingredient;
  }
}

export const ingredientMco = new IngredientMco();

