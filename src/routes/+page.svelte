<script lang="ts">
    import { onMount } from 'svelte';
    import taylor_test_kit from '$lib/images/taylor-test-kit.png';
    import cartridge_filter from '$lib/images/cartridge-filter.png';
    import pool_washing from '$lib/images/pool-pressure-washing.jpg';

    let services = [
        {
            title: "Water Analysis & Chemical Additions",
            description: "Our comprehensive water analysis service ensures the safety and clarity of your pool. We test critical water metrics including pH, CYA (cyanuric acid), FC (free chlorine), and salt levels, adjusting the tests as needed to cover any specific concerns. \n\nAfter identifying any water quality issues, we add the necessary chemicals to restore balance and maintain optimal conditions for your pool.",
            image: taylor_test_kit
        },
        {
            title: "Filter Cleaning",
            description: "Maintaining clean filters is essential for a healthy pool. Our service includes thorough cleaning of the skimmer and pump baskets, and we also wash cartridge filters when necessary. This process ensures efficient filtration and extends the life of your pool equipment.",
            image: cartridge_filter
        },
        {
            title: "Monthly Maintenance",
            description: "Regular maintenance keeps your pool in top condition. Our monthly service includes light cleaning tasks such as washing the pool walls, removing debris, and checking for potential issues. We also clean your saltwater chlorinator cell to ensure it operates at maximum efficiency, helping to prevent long-term problems and maintain water quality.",
            image: pool_washing
        }
    ];

    let pricing = [
        {
            plan: "Frequent Pool-users - Twice a Week",
            price: "$50 a visit",
            alt_price: "$240/month",
            description: "Ideal for frequent pool users and those with pools that require more intensive care, whether very large or very small. This plan ensures your pool remains in optimal condition with frequent maintenance and monitoring.",
            responsibilities: "Includes comprehensive water testing twice a week, purchasing and adding necessary chemicals, and performing minor cleaning tasks. This plan ensures consistent water balance and addresses any issues promptly to maintain a pristine pool environment.",
            color: "red"
        },
        {
            plan: "Once a Week",
            price: "$70/month",
            description: "Perfect for regular maintenance, this plan keeps your pool balanced and clean with weekly check-ups. It's a convenient option for maintaining pool health and preventing any major issues.",
            responsibilities: "Includes weekly chemical testing and balancing, ensuring your pool's water quality is always optimal. Additionally, this plan covers a one-time contaminant check per week to identify and address any potential problems early on.",
            color: "blue"
        },
        {
            plan: "Once a Month",
            price: "$30/month",
            description: "Suitable for low-maintenance pools or for those who prefer minimal intervention, this basic plan provides essential care to keep your pool's water in check with monthly assessments.",
            responsibilities: "Includes monthly chemical testing and balancing to maintain water quality. This plan does not cover any cleaning tasks, focusing solely on ensuring that the chemical levels are appropriate and safe.",
            color: "green"
        }
    ];

    let git_commit: string;
    onMount(async () => {
        const res = await fetch('https://api.github.com/repos/LimesKey/PoolMilton/commits?per_page=1');
        const data = await res.json();
        git_commit = data[0].commit.message;
    });
</script>

<style>
    @import url('https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&family=Lobster&display=swap');

    :global(body) {
        margin: 0;
        font-family: 'Roboto', sans-serif;
        background-color: #121212;
        color: #f1f1f1;
    }
    header {
        background: #1e88e5;
        padding: 1rem 0;
        text-align: center;
        color: white;
    }
    nav a {
        margin: 0 15px;
        color: white;
        text-decoration: none;
        font-weight: bold;
    }
    .title {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 80vh;
        background: linear-gradient(135deg, rgba(30, 136, 229, 0.8) 0%, rgba(66, 165, 245, 0.8) 100%), url('/src/assets/pool-side.jpg') no-repeat center center;
        background-size: cover;
        color: white;
        text-align: center;
    }
    .title h1 {
        font-family: 'Lobster', cursive;
        font-size: 3.5rem;
        margin-bottom: 0.5rem;
    }
    .title p {
        font-size: 1.5rem;
        margin-bottom: 2rem;
    }
    .button {
        padding: 1rem 2rem;
        opacity: 0.95;
        background: white;
        color: #1e88e5;
        border: 0.5px solid white;
        border-radius: 25px;
        cursor: pointer;
        font-size: 1.2rem;
        transition: all 0.4s ease-out;
    }
    .button:hover {
        background: #deebcc;
        opacity: 0.9;
    }
    .section {
        padding: 4rem 2rem;
    }
    .dark-section {
        background: #212121;
    }
    .section h2 {
        text-align: center;
        margin-bottom: 3rem;
        font-family: 'Lobster', cursive;
        font-size: 2.5rem;
    }
    .item {
        background: #333;
        padding: 2rem;
        border-radius: 10px;
        margin-bottom: 2rem;
        transition: transform 0.3s;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    .item:hover {
        transform: translateY(-10px);
    }
    .item h3 {
        margin-bottom: 1rem;
        font-family: 'Lobster', cursive;
        font-size: 1.8rem;
    }
    .pricing-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        max-width: 40%;
        margin-left: auto;
        margin-right: auto;
    }

    .pricing-item-background {
        padding: 2rem;
        border-radius: 10px;
        margin-bottom: 1rem;
    }
    .pricing-item {
        background: #333;
        border-radius: 10px;
        margin-bottom: 2rem;
        transition: transform 0.3s;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    .alt_pricing {
        font-size: 0.9em;
        color: rgb(146, 128, 128);
    }
    .service-item img {
        border-radius: 10px;
        margin-left: 2rem;
        margin-bottom: 1rem;
        max-width: 150px;
        max-height: 150px;
        object-fit: cover;
    }
    .service-item {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        max-width: 65%;
        margin-left: auto;
        margin-right: auto;
        text-align: center;
    }
    .service-item-content {
        flex: 1;
    }
    .misc {
        font-style: italic;
        font-size: small;
        padding: 1%;
    }
</style>

<header>
    <nav>
        <a href="#home">Home</a>
        <a href="#services">Services</a>
        <a href="#pricing">Pricing</a>
        <a href="#contact">Contact</a>
    </nav>
</header>

<section id="home" class="title">
    <h1>Milton Pool Water Testing</h1>
    <p>Ensuring your pool water is safe, clean, and crystal clear.</p>
    <a href="https://google.com">
        <input type="button" value="Sign Up" class="button"/>
    </a>
</section>

<section id="services" class="section dark-section">
    <h2>Our Services</h2>
    {#each services as service}
        <div class="item service-item">
            <img src={service.image} alt={service.title} />
            <div class="service-item-content">
                <h3>{service.title}</h3>
                <span style="white-space: pre-line">{service.description}</span>
            </div>
        </div>
    {/each}
</section>

<hr>

<section id="pricing" class="section dark-section">
    <h2>Pricing Options</h2>
    <div class="pricing-container">
        {#each pricing as plan}
            <div class="pricing-item">
                <div class="pricing-item-background" style="background-image: linear-gradient({plan.color}, white);">
                    <h3>{plan.plan}</h3>
                    <p>{plan.price}</p>
                    <div class="alt_pricing">
                        {plan.alt_price}
                    </div>
                    <p>{plan.description}</p>
                    <p><strong>Responsibilities:</strong> {plan.responsibilities}</p>
                </div>
            </div>
        {/each}
    </div>
</section>

<section id="contact" class="section">
    <h2>Contact Us</h2>
    <p>Email: info@ryandl.com</p>
    <p>Phone: (647)-803-8955</p>
    <br>
    <div class="misc">
        <!--
            <div id="time">TimeUndefined</div>
        -->
        <p>Commit: {git_commit}</p>
    </div>
</section>
