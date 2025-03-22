# OrderUpdateBodyOrderUpdateFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**side** | Option<**String**> | The trade side. | [optional]
**size** | Option<**f64**> | The quantity of the traded instrument. | [optional]
**instrument** | Option<[**models::OrderUpdateBodyOrderUpdateFieldsInstrument**](OrderUpdateBody_OrderUpdateFields_instrument.md)> |  | [optional]
**status** | Option<**String**> | The status or condition of the order. | [optional]
**strategy** | Option<**String**> | The name of the algorithm or the execution strategy. | [optional]
**timestamp** | Option<**f64**> | The epoch timestamp in milliseconds of the order update snapshot. | [optional]
**notes** | Option<**String**> | Free form input for comments/special order instructions. | [optional]
**client** | Option<**String**> | The client whose trade is being updated upon. | [optional]
**broker** | Option<**String**> | The broker involved in the order. | [optional]
**filled_quantity** | Option<**f64**> | How much of the trade is done. | [optional]
**time_in_force** | Option<**String**> | How long the order will remain active before it is executed or expires. | [optional]
**trader** | Option<**String**> | The trader involved in the order. | [optional]
**max_participation_percentage** | Option<**String**> | How much of the average daily volume in that name are you willing to be accountable for?. | [optional]
**benchmark** | Option<**String**> | Reference instrument, basket, index, price or benchmark bond. | [optional]
**account** | Option<**String**> | Trading account/client portfolio ID. | [optional]
**venue** | Option<**String**> | Exchange, ATS, dark pool - where the trade is being executed. | [optional]
**total_market_value** | Option<**f64**> | Approximate value of total order, or gross risk for undefined quantity of instrument. | [optional]
**average_price** | Option<**f64**> | Weighted average price of executed volume | [optional]
**limit_price** | Option<**f64**> | The limit price for the order. Do not execute higher (buy) or lower (sell) than this | [optional]
**target_price** | Option<**f64**> | Target execution price | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


