<script>
    import TablePanel from "../../lib/tables/TablePanel.svelte";
    let data=[
        {
            "job_no":"230400343HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400343HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400343HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400343HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
        {
            "job_no":"230400343HZH",
            "quotation_to":"浙江东星电子有限公司",
            "engineers":"Meng Wang",
            "job_creator":"Rachel Pan",
            "services":"ETL,cETL",
            "models":"dfdka adfk ;dkfj ;alkj dfa;lkdj ;kdf; ad f",
            "fee":13435
        },
    ];
    let configs=[
        {header:"Job No",values:["job_no"]},
        {header:"Client",values:["quotation_to"]},
        {header:"Engineers",values:["engineers"]},
        {header:"Services",values:["services"]},
        {header:"Models",values:["models"]},
        {header:"Fee",values:["fee"]}
    ]
</script>


<TablePanel rows={data} columnsConfig={configs}
on:item_db_click={(e)=>console.log(e.detail.row)}
/>