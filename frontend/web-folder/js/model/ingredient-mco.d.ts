export interface Ingredient {
    id: number;
    name: string;
    quantity: string;
}
export declare type IngredientPatch = Partial<Omit<Ingredient, 'id'>>;
declare class IngredientMco {
    list(): Promise<Ingredient[]>;
}
export declare const ingredientMco: IngredientMco;
export {};
