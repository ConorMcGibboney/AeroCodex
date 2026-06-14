# Bio-Regenerative Life Support Plan

## Intent

AeroCodex will include bio-regenerative life support systems as a first-class engineering math category, but it will begin conservatively with mass, energy, gas, water, nutrient, buffer, and reliability balance equations.

Phase 0.001 does **not** claim to model an actual certified life-support system. It only establishes typed Rust mass-balance primitives and the source-verification pipeline.

## Starter scope

Phase 0.001 starter equations:

```text
bioregen.mass_balance.closed_loop_fraction
bioregen.production.required_area
bioregen.buffer.residence_time
```

These equations support early reasoning about loop closure, required production area, and buffer duration.

## Research source targets

Primary research targets include:

- NASA Life Support Baseline Values and Assumptions Document (BVAD) records;
- NASA ECLSS and Advanced Life Support reports;
- NASA controlled ecological life support system / CELSS research;
- crop growth chamber and biomass production studies;
- waste processing, water recovery, oxygen generation, carbon dioxide removal, and nutrient loop studies;
- mission architecture documents where assumptions are explicit.

## Subdomains

```text
crew metabolic loads
oxygen generation
carbon dioxide removal and utilization
water recovery and purification
food/crop production
inedible biomass and waste processing
nutrient recycling
buffer sizing
loop closure fraction
energy balance and lighting loads
system reliability and redundancy
microbial/algal bioreactors
plant growth modules
air revitalization integration
thermal/humidity control coupling
```

## Evidence requirements

No bio-regenerative model should become stable unless it states:

1. crew size and metabolic assumptions;
2. mission duration assumptions;
3. biological species or technology assumptions;
4. environment assumptions: pressure, oxygen partial pressure, CO2 partial pressure, humidity, lighting, temperature, nutrient delivery;
5. mass, energy, and volume accounting boundaries;
6. failure and buffer assumptions;
7. source data and uncertainty;
8. whether it is a generic mass balance, empirical correlation, or validated subsystem model.

## Post-1.0 expansion

Beyond v1.0, the bioregen module can expand into crop growth, oxygen productivity, edible biomass, nutrient closure, water loops, waste processing, and integrated habitat loops. That expansion must remain honest about biological variability and mission-specific validation.
