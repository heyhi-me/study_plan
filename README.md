# Stellar Notes DApp

**Study Plan** - Blockchain-Based Decentralized Note-Taking System

## Project Description

A decentralized study planning application built on the Stellar blockchain using Soroban smart contracts. This application enables users to manage their learning sessions on-chain, ensuring data integrity, transparency, and persistence without relying on a centralized server or database.

## Features

### Study Plan Management
- Create a new study plan with a subject name, deadline, number of sessions, session duration, and priority level.
- View all active study plans retrieved directly from on-chain storage.
- Delete a study plan by its unique ID.
### Session Tracking
- Mark individual study sessions as completed.
- Each plan tracks the number of completed sessions against the total target.
- Prevents over-completion — marking sessions beyond the total is automatically rejected by the contract.
### Progress Monitoring
- Calculate and display progress percentage (0–100) for each study plan.
- Visual progress bar for at-a-glance tracking.
### Priority Filtering
- Plans are categorized into three priority levels: Low, Medium, and High.
- Filter the plan list by priority level on the frontend without additional contract calls.
### On-chain Statistics
- Retrieve aggregated statistics including total plans, total sessions completed, and total sessions across all plans.
- Stats are computed directly by the contract and returned as a single structured response.

## ID Smart Contract Testnet

CCIINTLOIWWPVZXFI2UEB7PPB5U2MPCAM7RP7VJUTFU6HNSINJTSKWLR