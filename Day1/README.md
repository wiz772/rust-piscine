# DynamicArray : réimplémentation du comportement principal de `Vec<T>` from scratch

## Règles des invariants

1. La longueur (`len`) ne dépasse jamais la capacité (`capacity`) : `len <= capacity`.
2. Les éléments d'indices `0..len` sont initialisés et peuvent être lus ou détruits.
3. Les emplacements d'indices `len..capacity` ne sont pas considérés comme contenant des éléments initialisés.
4. Si `capacity > 0`, `ptr` pointe vers une allocation valide, correctement alignée et suffisamment grande pour contenir `capacity` éléments de type `T`.
5. Si `capacity == 0`, `ptr` peut être nul.
6. Chaque élément initialisé doit être détruit exactement une fois, et chaque allocation doit être libérée exactement une fois.