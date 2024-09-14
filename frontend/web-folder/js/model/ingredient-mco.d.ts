export interface Ingredient {
    id: number;
    name: string;
    quantity: string;
}
export declare type IngredientPatch = Partial<Omit<Ingredient, 'id'>>;
declare class IngredientMco {
    list(): Promise<Ingredient[]>;
    create(data: IngredientPatch): Promise<Ingredient>;
    update(id: number, data: IngredientPatch): Promise<Ingredient>;
    delete(id: number): Promise<Ingredient>;
}
export declare const ingredientMco: IngredientMco;
export {};
